use {
    crate::{
        cli::AppConfig,
        server::{Server, ServerList},
    },
    anyhow::{Result, bail},
    speedtest_rs::{speedtest, speedtest_config::SpeedTestConfig},
    tracing::{error, info, warn},
};

const MBPS_DIVISOR: f64 = 1_000_000.0;

/// Specifies which part of the speed test to perform.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Test download speed only.
    Download,
    /// Test upload speed only.
    Upload,
    /// Test both download and upload speeds (default).
    #[default]
    Both,
}

/// A single speed measurement result.
#[derive(Debug, Clone)]
pub struct SpeedMeasurement {
    /// Speed in Megabits per second (Mbps).
    pub mbps: f64,
}

/// The complete result of a speed test session.
#[derive(Debug, Clone)]
pub struct SpeedTestResult {
    /// The server used for testing.
    pub server: Server,
    /// Result of the download test, if performed.
    pub download: Option<SpeedMeasurement>,
    /// Result of the upload test, if performed.
    pub upload: Option<SpeedMeasurement>,
}

/// A speed test executor.
pub struct SpeedTest {
    /// The server to test against.
    pub server: Server,
    /// The direction(s) to test.
    pub direction: Direction,
}

impl SpeedTest {
    /// Creates a new `SpeedTest` instance.
    #[must_use]
    pub const fn new(server: Server, direction: Direction) -> Self {
        Self { server, direction }
    }

    /// Runs the configured speed test.
    ///
    /// # Errors
    /// Returns an error if the speedtest configuration fails or the test itself fails.
    pub fn run<F>(&self, progress_callback: F) -> Result<SpeedTestResult>
    where 
        F: Fn() + Send + Copy + Sync + 'static,
    {
        let mut speed_config = speedtest::get_configuration()
            .map_err(|e| anyhow::anyhow!("Failed to retrieve speedtest configuration: {e:?}"))?;

        self.run_test(&mut speed_config, progress_callback)
    }

    const fn should_download(&self) -> bool {
        matches!(self.direction, Direction::Download | Direction::Both)
    }

    const fn should_upload(&self) -> bool {
        matches!(self.direction, Direction::Upload | Direction::Both)
    }

    fn run_test<F>(&self, config: &mut SpeedTestConfig, progress_callback: F) -> Result<SpeedTestResult> 
    where 
        F: Fn() + Send + Copy + Sync + 'static, 
    {
        let server = self.server.to_speedtest_server();
        info!("Testing connection on server: {} ({})", server.id, server.name);

        let download = if self.should_download() {
            Some(self.run_download_test(config, progress_callback)?)
        } else {
            None
        };

        let upload = if self.should_upload() {
            Some(self.run_upload_test(config, progress_callback)?)
        } else {
            None
        };

        Ok(SpeedTestResult {
            server: self.server.clone(),
            download,
            upload,
        })
    }

    /// Performs a download speed test specifically.
    ///
    /// # Errors
    /// Returns an error if the download test fails.
    pub fn run_download_test<F>(
        &self, 
        config: &mut SpeedTestConfig, 
        progress_callback: F
    ) -> Result<SpeedMeasurement> 
    where 
        F: Fn() + Send + Sync + 'static,
    {
        let server = self.server.to_speedtest_server();
        info!("Performing download speed test...");

        let measurement = speedtest::test_download_with_progress_and_config(
            &server,
            progress_callback,
            config,
        )
        .map_err(|e| anyhow::anyhow!("Download speed test failed: {e:?}"))?;

        Ok(SpeedMeasurement { 
            mbps: Self::calculate_mbps(measurement.bps_f64()) 
        })
    }

    fn run_upload_test<F>(&self, config: &SpeedTestConfig, progress_callback: F) -> Result<SpeedMeasurement> 
    where 
        F: Fn() + Send + Copy + Sync + 'static,
    {
        let server = self.server.to_speedtest_server();
        info!("Performing upload speed test...");
        
        let measurement = speedtest::test_upload_with_progress_and_config(
            &server,
            progress_callback,
            config,
        )
        .map_err(|e| anyhow::anyhow!("Upload speed test failed: {e:?}"))?;
        
        Ok(SpeedMeasurement { 
            mbps: Self::calculate_mbps(measurement.bps_f64()) 
        })
    }

    const fn calculate_mbps(bps: f64) -> f64 {
        bps / MBPS_DIVISOR
    }

    /// Selects a server and executes the speed test.
    ///
    /// # Errors
    /// Returns an error if no servers are available or all attempts fail.
    pub fn execute<F>(config: &AppConfig, progress_callback: F) -> Result<SpeedTestResult> 
    where 
        F: Fn() + Send + Copy + Sync + 'static,
    {
        let servers = ServerList::select_server(config.server_id().cloned())?;

        if servers.is_empty() {
            error!("No servers available for testing");
            bail!("No servers available for testing");
        }

        for (index, server) in servers.iter().enumerate() {
            let test = Self::new(server.clone(), config.direction());
            match test.run(progress_callback) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    error!("Error with server {}: {}", server.id, e);
                    if index < servers.len() - 1 {
                        warn!("Trying next server...");
                    } else {
                        let msg = "All attempts failed. Please check your connection.";
                        error!("{msg}");
                        bail!("{msg}");
                    }
                }
            }
        }
        bail!("Unexpected error: loop finished without success or bail")
    }
}
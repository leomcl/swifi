use {
        crate::{
                cli::Config,
                server::{Server, ServerList},
        },
        anyhow::{Result, bail},
        speedtest_rs::{
                speedtest::{self, SpeedTestServer},
                speedtest_config::SpeedTestConfig,
        },
        std::io::{self, Write},
        tracing::{error, info, warn},
        tracing_subscriber::fmt::Subscriber,
};

const MBPS_DIVISOR: f64 = 1_000_000.0;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum TestDirection {
        Download,
        Upload,
        #[default]
        Both,
}

pub struct Test {
        pub server: Server,
        pub direction: TestDirection,
}

impl Test {
        /// Create a new Test instance
        pub fn new(server: Server, direction: TestDirection) -> Self {
                Self { server, direction }
        }

        /// Run the test on the configured server
        pub fn run(&self) -> Result<()> {
                let subscriber =
                        Subscriber::builder().with_writer(io::stdout).finish();
                let _ = tracing::subscriber::set_global_default(subscriber);

                let mut speed_config =
                        speedtest::get_configuration().map_err(|e| {
                                anyhow::anyhow!("Failed to get config: {:?}", e)
                        })?;

                let st_server = self.server.to_speedtest_server();
                self.run_test(&st_server, &mut speed_config)
        }

        fn should_download(&self) -> bool {
                matches!(
                        self.direction,
                        TestDirection::Download | TestDirection::Both
                )
        }

        fn should_upload(&self) -> bool {
                matches!(
                        self.direction,
                        TestDirection::Upload | TestDirection::Both
                )
        }

        /// # Errors
        /// Will return `Err` if either the download or upload test fails.
        fn run_test(
                &self,
                server: &SpeedTestServer,
                config: &mut SpeedTestConfig,
        ) -> Result<()> {
                info!(
                        "Testing connection on server: {} ({})",
                        server.id, server.name
                );

                if self.should_download() {
                        self.run_download_test(server, config)?;
                }

                if self.should_upload() {
                        self.run_upload_test(server, config)?;
                }

                Ok(())
        }

        /// # Errors
        /// Will return `Err` if the download test fails.
        fn run_download_test(
                &self,
                server: &SpeedTestServer,
                config: &mut SpeedTestConfig,
        ) -> Result<()> {
                info!("Performing download speed test...");
                let measurement = speedtest::test_download_with_progress_and_config(
            server,
            || {
                print!("#");
                io::stdout().flush().unwrap();
            },
            config,
        )
        .map_err(|e| anyhow::anyhow!("Download test failed, try another server: {e:?}"))?;
                let download_mbps = Self::calculate_mbps(measurement.bps_f64());
                info!("Download Speed: {download_mbps:.2} Mbps");
                Ok(())
        }

        /// # Errors
        /// Will return `Err` if the upload test fails.
        fn run_upload_test(
                &self,
                server: &SpeedTestServer,
                config: &SpeedTestConfig,
        ) -> Result<()> {
                info!("Performing upload speed test...");
                let measurement = speedtest::test_upload_with_progress_and_config(
            server,
            || {
                print!("#");
                if let Err(e) = io::stdout().flush() {
                    error!("Failed to flush stdout: {}", e);
                                        }
                        },
                        
            config,
        )
        .map_err(|e| anyhow::anyhow!("Upload test failed, try another server: {e:?}"))?;
                let upload_mbps = Self::calculate_mbps(measurement.bps_f64());
                info!("Upload Speed: {upload_mbps:.2} Mbps");
                Ok(())
        }

        /// Asume input is BPS, maybe use units in future.  
        fn calculate_mbps(bps: f64) -> f64 {
                bps / MBPS_DIVISOR
        }
}

/// # Errors
/// Will return `Err` if no servers are available or all tests fail.
pub fn do_test_config(config: &Config) -> Result<()> {
        let subscriber = Subscriber::builder().with_writer(io::stdout).finish();
        let _ = tracing::subscriber::set_global_default(subscriber);

        let servers = ServerList::select_server(config.server.clone())?;

        if servers.is_empty() {
                error!("No servers available for testing");
                bail!("No servers available for testing");
        }

        for (index, server) in servers.iter().enumerate() {
                let test = Test::new(server.clone(), config.direction);
                match test.run() {
                        Ok(()) => return Ok(()),
                        Err(e) => {
                                error!(
                                        "Error with server {}: {}",
                                        server.id, e
                                );
                                if index < servers.len() - 1 {
                                        warn!("Trying next server...");
                                } else {
                                        error!(
                                                "All attempts failed. Please check your connection."
                                        );
                                        bail!(
                                                "All attempts failed. Please check your connection."
                                        );
                                }
                        }
                }
        }
        Ok(())
}

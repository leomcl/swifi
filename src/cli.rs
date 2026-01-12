use {crate::speed_test, clap::Parser};

/// A CLI tool for testing wifi download and upload speeds.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// List available servers sorted by distance
    #[arg(short, long)]
    pub list: bool,

    /// Specify a specific server ID to use
    #[arg(short, long)]
    pub server: Option<String>,

    /// Perform a download speed test
    #[arg(short, long)]
    pub down: bool,

    /// Perform an upload speed test
    #[arg(short, long)]
    pub up: bool,
}

/// Configuration for the application, defining parameters for the speed test.
#[derive(Debug, Default)]
pub struct AppConfig {
    list: bool,
    server: Option<String>,
    direction: speed_test::Direction,
}

impl AppConfig {
    /// Returns true if the user requested to list available servers.
    #[must_use]
    pub const fn has_list(&self) -> bool {
        self.list
    }

    /// Returns the specific server ID to test against, if requested.
    #[must_use]
    pub const fn server_id(&self) -> Option<&String> {
        self.server.as_ref()
    }

    /// Returns the direction of the test (Download, Upload, or Both).
    #[must_use]
    pub const fn direction(&self) -> speed_test::Direction {
        self.direction
    }
}

/// Builder for creating an `AppConfig` from CLI arguments.
pub struct AppConfigBuilder {
    list: bool,
    server: Option<String>,
    down: bool,
    up: bool,
}

impl AppConfigBuilder {
    /// Creates a new builder from the parsed command-line arguments.
    #[must_use]
    pub fn from_args(args: CliArgs) -> Self {
        Self {
            list: args.list,
            server: args.server,
            down: args.down,
            up: args.up,
        }
    }

    /// Consumes the builder and returns the final `AppConfig`.
    #[must_use]
    pub fn build(self) -> AppConfig {
        let direction = match (self.down, self.up) {
            (true, false) => speed_test::Direction::Download,
            (false, true) => speed_test::Direction::Upload,
            (true, true) | (false, false) => speed_test::Direction::Both,
        };
        AppConfig {
            list: self.list,
            server: self.server,
            direction,
        }
    }
}
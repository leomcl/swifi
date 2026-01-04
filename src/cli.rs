use {crate::test, clap::Parser};

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

#[derive(Debug, Default)]
pub struct Config {
        pub list: bool,
        pub server: Option<String>,
        pub direction: test::TestDirection,
}

pub struct ConfigBuilder {
        list: bool,
        server: Option<String>,
        down: bool,
        up: bool,
}

impl ConfigBuilder {
        pub fn from_args(args: CliArgs) -> Self {
                Self {
                        list: args.list,
                        server: args.server,
                        down: args.down,
                        up: args.up,
                }
        }

        pub fn build(self) -> Config {
                let direction = match (self.down, self.up) {
                        (true, false) => test::TestDirection::Download,
                        (false, true) => test::TestDirection::Upload,
                        (true, true) | (false, false) => {
                                test::TestDirection::Both
                        }
                };
                Config {
                        list: self.list,
                        server: self.server,
                        direction,
                }
        }
}

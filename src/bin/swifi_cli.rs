/// CLI entry point for the swifi speed test tool.
use {
        anyhow::Result,
        clap::Parser,
        swifi::{CliArgs, ConfigBuilder, ServerList, do_test_config},
};

fn main() -> Result<()> {
        let args = CliArgs::parse();
        let config = ConfigBuilder::from_args(args).build();

        if config.list {
                let server_list = ServerList::list_servers()?;
                println!("Available Servers:");
                println!(
                        "{:<10} {:<30} {:<40} {:<10}",
                        "ID", "Sponsor", "Name", "Distance"
                );
                println!("{}", "-".repeat(100));
                for server in server_list.servers {
                        println!(
                                "{:<10} {:<30} {:<40} {:<10.2}",
                                server.id,
                                server.sponsor,
                                server.name,
                                server.distance_km
                        );
                }
                return Ok(());
        }

        if let Err(e) = do_test_config(&config) {
                eprintln!("Error: {e}");
        }
        Ok(())
}

//! CLI entry point for the swifi speed test tool.
use {
    anyhow::Result, clap::Parser, std::io::Write, swifi::{CliArgs, ConfigBuilder, ServerList, Test}
};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .init();

    let args = CliArgs::parse();
    let config = ConfigBuilder::from_args(args).build();

    if config.has_list() {
        let server_list = ServerList::list_servers()?;
        print!("{}", server_list.format_table());
        return Ok(());
    }

    Test::execute(&config, || {
        print!("#");
        let _ = std::io::stdout().flush();
    })?;
    Ok(())
}

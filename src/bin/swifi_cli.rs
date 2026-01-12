//! CLI entry point for the swifi speed test tool.
use {
    anyhow::Result,
    clap::Parser,
    std::io::Write,
    swifi::{AppConfigBuilder, CliArgs, ServerList, SpeedTest},
};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .init();

    let args = CliArgs::parse();
    let config = AppConfigBuilder::from_args(args).build();

    if config.has_list() {
        let server_list = ServerList::list_servers()?;
        print!("{}", server_list.format_table());
        return Ok(());
    }

    let result = SpeedTest::execute(&config, || {
        print!("#");
        let _ = std::io::stdout().flush();
    })?;

    println!(); 

    if let Some(down) = result.download {
        tracing::info!("Download Speed: {:.2} Mbps", down.mbps);
    }

    if let Some(up) = result.upload {
        tracing::info!("Upload Speed: {:.2} Mbps", up.mbps);
    }
    
    Ok(())
}
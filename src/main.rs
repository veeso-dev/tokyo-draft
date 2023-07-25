#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

use std::io::Write;
use std::path::Path;

mod args;
mod config;
mod hook;
mod render;
mod web;

const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args: args::Args = argh::from_env();
    tracing_subscriber::fmt::init();
    info!(
        "tokyo-draft v{} - developed by {}",
        APP_VERSION, APP_AUTHORS
    );
    let config = config::Config::try_from_env()?;
    info!("initializing web service...");
    let web_service = web::WebServer::init(
        &config.listener_address,
        config.render_hook,
        &config.templates_dir,
    )
    .await?;
    if let Some(pidfile) = args.pidfile.as_deref() {
        debug!("writing pidfile...");
        write_pidfile(pidfile)?;
    }
    info!("web service OK; running web server...");
    web_service.run().await?;

    Ok(())
}

#[cfg(target_family = "unix")]
fn write_pidfile(pidfile: &Path) -> anyhow::Result<()> {
    let pid = std::process::id();

    let mut f = std::fs::File::create(pidfile)?;
    writeln!(&mut f, "{pid}")?;

    Ok(())
}

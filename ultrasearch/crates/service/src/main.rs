//! Entry point for the UltraSearch Windows service (bootstrap only for now).

use anyhow::Result;
use core_types::config::load_config;

mod logging;
mod metrics;

fn main() -> Result<()> {
    let cfg = load_config(None)?;
    logging::init(&cfg.logging, &cfg.app.data_dir, "service")?;

    if cfg.metrics.enabled {
        let _metrics = metrics::init_metrics_from_config(&cfg.metrics)?;
        // TODO: wire metrics handle into IPC/server once implemented.
    }

    println!("UltraSearch service placeholder – wiring pending.");

    Ok(())
}

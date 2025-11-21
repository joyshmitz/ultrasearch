//! Entry point for the UltraSearch Windows service (bootstrap only for now).

use anyhow::Result;
use core_types::config::load_config;
use service::{
    init_tracing,
    metrics::{init_metrics_from_config, set_global_metrics},
};
use std::sync::Arc;

fn main() -> Result<()> {
    let cfg = load_config(None)?;
    init_tracing()?;

    if cfg.metrics.enabled {
        let metrics = Arc::new(init_metrics_from_config(&cfg.metrics)?);
        set_global_metrics(metrics);
    }

    println!("UltraSearch service placeholder – wiring pending.");

    Ok(())
}

//! Service support library: tracing/logging bootstrap and metrics helpers.

mod logging;
pub mod metrics;

pub use logging::init as init_tracing;
pub use metrics::{init_metrics_from_config, scrape_metrics, ServiceMetrics};

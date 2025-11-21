//! Service support library: tracing/logging bootstrap and metrics helpers.

pub mod bootstrap;

#[cfg(windows)]
pub mod windows;

pub use logging::{init_tracing, init_tracing_with_config};
pub use meta_ingest::{ingest_file_meta_batch, ingest_with_paths};
pub use metrics::{
    ServiceMetrics, ServiceMetricsSnapshot, init_metrics_from_config, scrape_metrics,
};
pub use priority::{ProcessPriority, set_process_priority};
pub use scheduler_runtime::SchedulerRuntime;
pub use search_handler::{
    MetaIndexSearchHandler, SearchHandler, StubSearchHandler, search, set_search_handler,
};
pub use status_provider::{
    BasicStatusProvider, init_basic_status_provider, set_status_provider, status_snapshot,
};

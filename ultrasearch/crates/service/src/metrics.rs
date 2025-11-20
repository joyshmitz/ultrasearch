use anyhow::Result;
use once_cell::sync::Lazy;
use prometheus::{
    opts, Encoder, Histogram, HistogramOpts, IntCounter, Registry, TextEncoder,
};

/// Shared metrics handle for the service.
pub struct ServiceMetrics {
    pub registry: Registry,
    pub requests_total: IntCounter,
    pub request_latency: Histogram,
    pub worker_failures: IntCounter,
}

impl ServiceMetrics {
    pub fn new() -> Result<Self> {
        let registry = Registry::new();

        let requests_total =
            IntCounter::with_opts(opts!("requests_total", "Total IPC requests served"))?;
        let request_latency = Histogram::with_opts(HistogramOpts::new(
            "request_latency_seconds",
            "IPC request latency in seconds",
        ))?;
        let worker_failures =
            IntCounter::with_opts(opts!("worker_failures_total", "Index worker failures"))?;

        registry.register(Box::new(requests_total.clone()))?;
        registry.register(Box::new(request_latency.clone()))?;
        registry.register(Box::new(worker_failures.clone()))?;

        Ok(Self {
            registry,
            requests_total,
            request_latency,
            worker_failures,
        })
    }
}

static ENCODER: Lazy<TextEncoder> = Lazy::new(TextEncoder::new);

pub fn init_metrics() -> Result<ServiceMetrics> {
    ServiceMetrics::new()
}

/// Encode all metrics in Prometheus text format.
pub fn scrape_metrics(metrics: &ServiceMetrics) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let metric_families = metrics.registry.gather();
    ENCODER.encode(&metric_families, &mut buffer)?;
    Ok(buffer)
}

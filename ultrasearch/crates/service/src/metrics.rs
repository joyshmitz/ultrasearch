use anyhow::Result;
use core_types::config::MetricsSection;
use once_cell::sync::Lazy;
use prometheus::{opts, Encoder, Histogram, HistogramOpts, IntCounter, Registry, TextEncoder};

/// Shared metrics handle for the service.
pub struct ServiceMetrics {
    pub registry: Registry,
    pub requests_total: IntCounter,
    pub request_latency: Histogram,
    pub worker_failures: IntCounter,
    pub worker_failure_threshold: u64,
}

impl ServiceMetrics {
    pub fn new(cfg: &MetricsSection) -> Result<Self> {
        let registry = Registry::new();

        let requests_total =
            IntCounter::with_opts(opts!("requests_total", "Total IPC requests served"))?;
        let mut hist_opts =
            HistogramOpts::new("request_latency_seconds", "IPC request latency in seconds");
        if !cfg.request_latency_buckets.is_empty() {
            hist_opts = hist_opts.buckets(cfg.request_latency_buckets.clone());
        }
        let request_latency = Histogram::with_opts(hist_opts)?;
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
            worker_failure_threshold: cfg.worker_failure_threshold,
        })
    }
}

static ENCODER: Lazy<TextEncoder> = Lazy::new(TextEncoder::new);

pub fn init_metrics_from_config(cfg: &MetricsSection) -> Result<ServiceMetrics> {
    ServiceMetrics::new(cfg)
}

/// Encode all metrics in Prometheus text format.
pub fn scrape_metrics(metrics: &ServiceMetrics) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let metric_families = metrics.registry.gather();
    ENCODER.encode(&metric_families, &mut buffer)?;
    Ok(buffer)
}

use crate::metrics::global_metrics_snapshot;
use ipc::{MetricsSnapshot, VolumeStatus};
use std::sync::{Arc, OnceLock};

/// Snapshot of service status used by IPC responses.
#[derive(Debug, Clone)]
pub struct StatusSnapshot {
    pub volumes: Vec<VolumeStatus>,
    pub scheduler_state: String,
    pub metrics: Option<MetricsSnapshot>,
    pub last_index_commit_ts: Option<i64>,
}

pub trait StatusProvider: Send + Sync {
    fn snapshot(&self) -> StatusSnapshot;
}

static PROVIDER: OnceLock<Arc<dyn StatusProvider>> = OnceLock::new();

/// Install a process-wide status provider.
pub fn set_status_provider(provider: Arc<dyn StatusProvider>) {
    let _ = PROVIDER.set(provider);
}

/// Fetch the current snapshot from the registered provider (or a default stub).
pub fn status_snapshot() -> StatusSnapshot {
    if let Some(provider) = PROVIDER.get() {
        return provider.snapshot();
    }

    StatusSnapshot {
        volumes: Vec::new(),
        scheduler_state: "unknown".to_string(),
        metrics: global_metrics_snapshot(Some(0), Some(0)),
        last_index_commit_ts: None,
    }
}

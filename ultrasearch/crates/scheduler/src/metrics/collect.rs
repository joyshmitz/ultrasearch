use std::time::{Duration, Instant};
use sysinfo::System;

/// Snapshot of system load suitable for scheduling decisions.
#[derive(Debug, Clone, Copy)]
pub struct SystemLoad {
    pub cpu_percent: f32,
    pub mem_used_percent: f32,
    /// Aggregate disk throughput in bytes/sec since the previous sample.
    /// sysinfo 0.30 does not expose disk IO counters on `System`; keep field for forward compat.
    pub disk_bytes_per_sec: u64,
    pub disk_busy: bool,
    /// Duration covered by this sample (useful for metrics surfaces).
    pub sample_duration: Duration,
}

pub struct SystemLoadSampler {
    system: System,
    disk_busy_threshold_bps: u64,
    last_sample: Instant,
}

impl SystemLoadSampler {
    /// Create a sampler with a busy threshold expressed in bytes/sec.
    pub fn new(disk_busy_threshold_bps: u64) -> Self {
        let mut system = System::new();
        system.refresh_cpu();
        system.refresh_memory();

        Self {
            system,
            disk_busy_threshold_bps,
            last_sample: Instant::now(),
        }
    }

    pub fn disk_threshold(&self) -> u64 {
        self.disk_busy_threshold_bps
    }

    pub fn set_disk_threshold(&mut self, disk_busy_threshold_bps: u64) {
        self.disk_busy_threshold_bps = disk_busy_threshold_bps;
    }

    /// Refresh system metrics and compute load figures.
    pub fn sample(&mut self) -> SystemLoad {
        self.system.refresh_cpu();
        self.system.refresh_memory();

        let now = Instant::now();
        let elapsed = now.saturating_duration_since(self.last_sample);
        let elapsed = if elapsed.is_zero() {
            Duration::from_millis(1)
        } else {
            elapsed
        };

        let cpu_percent = self.system.global_cpu_info().cpu_usage();
        let total_mem = self.system.total_memory().max(1);
        let mem_used_percent = (self.system.used_memory() as f32 / total_mem as f32) * 100.0;

        // sysinfo currently lacks aggregate disk IO counters at the System level.
        // Keep the hook so we can enable it when available.
        let disk_bytes_per_sec = 0;
        let disk_busy = disk_bytes_per_sec >= self.disk_busy_threshold_bps;

        self.last_sample = now;

        SystemLoad {
            cpu_percent,
            mem_used_percent,
            disk_bytes_per_sec,
            disk_busy,
            sample_duration: elapsed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_busy_threshold_applied() {
        let mut sampler = SystemLoadSampler::new(1_000);
        let load = sampler.sample();
        let computed_flag = load.disk_bytes_per_sec >= sampler.disk_threshold();
        assert_eq!(load.disk_busy, computed_flag);
        assert!(load.sample_duration.as_millis() > 0);
    }
}

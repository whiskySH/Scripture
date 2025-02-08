use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct ProfilerMetrics {
    pub count: usize,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}

pub struct Profiler {
    metrics: Arc<RwLock<HashMap<String, ProfilerMetrics>>>,
    active_measurements: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            active_measurements: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_measurement(&self, name: &str) {
        let mut measurements = self.active_measurements.write().await;
        measurements.insert(name.to_string(), Instant::now());
    }

    pub async fn end_measurement(&self, name: &str) {
        let mut measurements = self.active_measurements.write().await;
        let mut metrics = self.metrics.write().await;
        
        if let Some(start_time) = measurements.remove(name) {
            let duration = start_time.elapsed();
            
            metrics
                .entry(name.to_string())
                .and_modify(|m| {
                    m.count += 1;
                    m.total_time += duration;
                    m.min_time = std::cmp::min(m.min_time, duration);
                    m.max_time = std::cmp::max(m.max_time, duration);
                })
                .or_insert(ProfilerMetrics {
                    count: 1,
                    total_time: duration,
                    min_time: duration,
                    max_time: duration,
                });
        }
    }

    pub async fn get_metrics(&self) -> HashMap<String, ProfilerMetrics> {
        self.metrics.read().await.clone()
    }
}
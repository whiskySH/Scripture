#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_profiler_measurements() {
        let profiler = Profiler::new();
        
        profiler.start_measurement("test_op").await;
        sleep(Duration::from_millis(100)).await;
        profiler.end_measurement("test_op").await;
        
        let metrics = profiler.get_metrics().await;
        let test_metrics = metrics.get("test_op").unwrap();
        
        assert_eq!(test_metrics.count, 1);
        assert!(test_metrics.total_time >= Duration::from_millis(100));
    }
}
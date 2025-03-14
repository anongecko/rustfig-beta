use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;

static GLOBAL_METRICS: Lazy<Arc<Mutex<HashMap<String, ComponentMetrics>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Stores metrics for a component
#[derive(Clone)]
pub struct ComponentMetrics {
    name: String,
    operation_count: Arc<AtomicUsize>,
    operation_metrics: Arc<Mutex<HashMap<String, OperationMetrics>>>,
}

/// Metrics for a specific operation
#[derive(Clone, Debug)]
pub struct OperationMetrics {
    name: String,
    call_count: usize,
    total_duration: Duration,
    min_duration: Duration,
    max_duration: Duration,
}

impl OperationMetrics {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            call_count: 0,
            total_duration: Duration::from_nanos(0),
            min_duration: Duration::from_secs(u64::MAX),
            max_duration: Duration::from_nanos(0),
        }
    }
    
    fn update(&mut self, duration: Duration) {
        self.call_count += 1;
        self.total_duration += duration;
        self.min_duration = self.min_duration.min(duration);
        self.max_duration = self.max_duration.max(duration);
    }
    
    /// Get average duration
    pub fn avg_duration(&self) -> Duration {
        if self.call_count == 0 {
            Duration::from_nanos(0)
        } else {
            self.total_duration / self.call_count as u32
        }
    }
}

/// Performance measurement utility
pub struct PerformanceMetrics {
    component_name: String,
    metrics: ComponentMetrics,
}

impl PerformanceMetrics {
    pub fn new(component_name: &str) -> Self {
        let metrics = {
            let mut global = GLOBAL_METRICS.lock().unwrap();
            
            match global.get(component_name) {
                Some(metrics) => metrics.clone(),
                None => {
                    let metrics = ComponentMetrics {
                        name: component_name.to_string(),
                        operation_count: Arc::new(AtomicUsize::new(0)),
                        operation_metrics: Arc::new(Mutex::new(HashMap::new())),
                    };
                    
                    global.insert(component_name.to_string(), metrics.clone());
                    metrics
                }
            }
        };
        
        Self {
            component_name: component_name.to_string(),
            metrics,
        }
    }
    
    /// Measure the execution time of an operation
    pub fn measure_operation<'a>(&'a self, operation_name: &'a str) -> OperationTimer<'a> {
        self.metrics.operation_count.fetch_add(1, Ordering::Relaxed);
        
        OperationTimer {
            start: Instant::now(),
            metrics: &self.metrics,
            operation_name,
        }
    }
    
    /// Get metrics for component
    pub fn get_metrics(&self) -> HashMap<String, OperationMetrics> {
        let metrics_lock = self.metrics.operation_metrics.lock().unwrap();
        metrics_lock.clone()
    }
    
    /// Get total operation count
    pub fn get_operation_count(&self) -> usize {
        self.metrics.operation_count.load(Ordering::Relaxed)
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        self.metrics.operation_count.store(0, Ordering::Relaxed);
        let mut metrics_lock = self.metrics.operation_metrics.lock().unwrap();
        metrics_lock.clear();
    }
}

/// Timer for an operation
pub struct OperationTimer<'a> {
    start: Instant,
    metrics: &'a ComponentMetrics,
    operation_name: &'a str,
}

impl<'a> Drop for OperationTimer<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        
        let mut metrics_lock = self.metrics.operation_metrics.lock().unwrap();
        
        let operation_metrics = metrics_lock
            .entry(self.operation_name.to_string())
            .or_insert_with(|| OperationMetrics::new(self.operation_name));
        
        operation_metrics.update(duration);
    }
}

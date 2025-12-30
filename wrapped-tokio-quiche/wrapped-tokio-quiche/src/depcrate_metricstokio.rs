// Generated macro for tokio (module)
macro_rules! Depcrate_metricstokio {
() => {
// Module: crate::metrics
// Provides: {"tokio"}
// Dependencies: {}
# [metrics] mod tokio { # [doc = " Histogram of task schedule delays"] # [ctor = HistogramBuilder { buckets : & [0.0 , 1E-4 , 2E-4 , 3E-4 , 4E-4 , 5E-4 , 6E-4 , 7E-4 , 8E-4 , 9E-4 , 1E-3 , 1E-2 , 2E-2 , 4E-2 , 8E-2 , 1E-1 , 1.0] , }] pub fn runtime_task_schedule_delay_histogram (task : & Arc < str >,) -> TimeHistogram ; # [doc = " Histogram of task poll durations"] # [ctor = HistogramBuilder { buckets : & [0.0 , 1E-4 , 2E-4 , 3E-4 , 4E-4 , 5E-4 , 6E-4 , 7E-4 , 8E-4 , 9E-4 , 1E-3 , 1E-2 , 2E-2 , 4E-2 , 8E-2 , 1E-1 , 1.0] , }] pub fn runtime_task_poll_duration_histogram (task : & Arc < str >) -> TimeHistogram ; # [doc = " Helps us get a rough idea of if our waker is causing issues."] pub fn runtime_task_total_poll_time_micros (task : & Arc < str >) -> Counter ; }
};
}

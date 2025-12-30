// Generated macro for spawn (function)
macro_rules! Depcrate_metrics_tokio_taskspawn {
() => {
// Module: crate::metrics::tokio_task
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Spawn a potentially instrumented task."] # [doc = ""] # [doc = " Depending on whether the `tokio-task-metrics` feature is enabled, this may"] # [doc = " instrument the task and collect metrics for it."] pub fn spawn < M , T > (name : & str , metrics : M , future : T) -> JoinHandle < T :: Output > where T : Future + Send + 'static , T :: Output : Send + 'static , M : Metrics , { let ctx = TelemetryContext :: current () ; if cfg ! (feature = "tokio-task-metrics") { tokio :: spawn (Instrumented :: new (name , metrics , ctx . apply (future))) } else { tokio :: spawn (ctx . apply (future)) } }
};
}

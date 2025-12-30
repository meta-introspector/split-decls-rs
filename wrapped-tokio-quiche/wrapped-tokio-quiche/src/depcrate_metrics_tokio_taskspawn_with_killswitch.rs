// Generated macro for spawn_with_killswitch (function)
macro_rules! Depcrate_metrics_tokio_taskspawn_with_killswitch {
() => {
// Module: crate::metrics::tokio_task
// Provides: {"spawn_with_killswitch"}
// Dependencies: {}
# [doc = " Spawn a potentially instrumented, long-lived task. Integrates with"] # [doc = " [task-killswitch](task_killswitch)."] # [doc = ""] # [doc = " Depending on whether the `tokio-task-metrics` feature is enabled, this may"] # [doc = " instrument the task and collect metrics for it."] pub fn spawn_with_killswitch < M , T > (name : & str , metrics : M , future : T) where T : Future < Output = () > + Send + 'static , M : Metrics , { let ctx = TelemetryContext :: current () ; if cfg ! (feature = "tokio-task-metrics") { killswitch_spawn (Instrumented :: new (name , metrics , ctx . apply (future))) ; } else { killswitch_spawn (ctx . apply (future)) ; } }
};
}

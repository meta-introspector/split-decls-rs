// Generated macro for Instrumented (struct)
macro_rules! Depcrate_metrics_tokio_taskInstrumented {
() => {
// Module: crate::metrics::tokio_task
// Provides: {"Instrumented"}
// Dependencies: {}
# [doc = " An instrumented future."] # [doc = ""] # [doc = " It's important to keep overhead low here, especially where contention is"] # [doc = " concerned."] # [pin_project] struct Instrumented < F , M > { # [pin] future : F , name : Arc < str > , timer : Arc < Mutex < Option < Instant > > > , metrics : M , }
};
}

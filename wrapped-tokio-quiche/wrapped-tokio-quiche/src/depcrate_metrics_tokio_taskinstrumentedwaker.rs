// Generated macro for InstrumentedWaker (struct)
macro_rules! Depcrate_metrics_tokio_taskInstrumentedWaker {
() => {
// Module: crate::metrics::tokio_task
// Provides: {"InstrumentedWaker"}
// Dependencies: {}
# [doc = " An instrumented waker for our instrumented future."] # [doc = ""] # [doc = " It's very important to keep overhead low here, especially where contention"] # [doc = " is concerned."] struct InstrumentedWaker { timer : Arc < Mutex < Option < Instant > > > , waker : Waker , }
};
}

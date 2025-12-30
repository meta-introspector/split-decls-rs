// Generated macro for impl_385 (impl)
macro_rules! Depcrate_metrics_tokio_taskimpl_385 {
() => {
// Module: crate::metrics::tokio_task
// Provides: {"impl_385"}
// Dependencies: {}
impl Wake for InstrumentedWaker { fn wake (self : Arc < Self >) { self . wake_by_ref () } fn wake_by_ref (self : & Arc < Self >) { { let mut guard = self . timer . lock () . unwrap () ; if guard . is_none () { * guard = Some (Instant :: now ()) } } self . waker . wake_by_ref () ; } }
};
}

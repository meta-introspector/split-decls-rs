// Generated macro for wake_by_ref (function)
macro_rules! Depcrate_taskwake_by_ref {
() => {
// Module: crate::task
// Provides: {"wake_by_ref"}
// Dependencies: {}
unsafe fn wake_by_ref (raw : * const ()) { let waker = from_raw (raw) ; waker . wake () ; mem :: forget (waker) ; }
};
}

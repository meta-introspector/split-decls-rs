// Generated macro for clone (function)
macro_rules! Depcrate_taskclone {
() => {
// Module: crate::task
// Provides: {"clone"}
// Dependencies: {}
unsafe fn clone (raw : * const ()) -> RawWaker { let waker = from_raw (raw) ; mem :: forget (waker . clone ()) ; to_raw (waker) }
};
}

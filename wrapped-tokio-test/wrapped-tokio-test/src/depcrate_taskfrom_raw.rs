// Generated macro for from_raw (function)
macro_rules! Depcrate_taskfrom_raw {
() => {
// Module: crate::task
// Provides: {"from_raw"}
// Dependencies: {}
unsafe fn from_raw (raw : * const ()) -> Arc < ThreadWaker > { Arc :: from_raw (raw as * const ThreadWaker) }
};
}

// Generated macro for to_raw (function)
macro_rules! Depcrate_taskto_raw {
() => {
// Module: crate::task
// Provides: {"to_raw"}
// Dependencies: {}
unsafe fn to_raw (waker : Arc < ThreadWaker >) -> RawWaker { RawWaker :: new (Arc :: into_raw (waker) as * const () , & VTABLE) }
};
}

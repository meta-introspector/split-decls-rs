// Generated macro for impl_27 (impl)
macro_rules! Depcrate_algorithms_hookimpl_27 {
() => {
// Module: crate::algorithms::hook
// Provides: {"impl_27"}
// Dependencies: {}
impl < D : DiffHook > NoFinishHook < D > { # [doc = " Wraps another hook."] pub fn new (d : D) -> NoFinishHook < D > { NoFinishHook (d) } # [doc = " Extracts the inner hook."] pub fn into_inner (self) -> D { self . 0 } }
};
}

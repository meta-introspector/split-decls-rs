// Generated macro for impl_127 (impl)
macro_rules! Depcrate_dispatcherimpl_127 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_127"}
// Dependencies: {}
# [cfg (feature = "std")] impl Drop for DefaultGuard { # [inline] fn drop (& mut self) { let prev = CURRENT_STATE . try_with (| state | state . default . replace (self . 0 . take ())) ; SCOPED_COUNT . fetch_sub (1 , Ordering :: Release) ; drop (prev) } }
};
}

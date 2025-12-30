// Generated macro for impl_127 (impl)
macro_rules! Depcrate_event_kqueueimpl_127 {
() => {
// Module: crate::event::kqueue
// Provides: {"impl_127"}
// Dependencies: {}
# [cfg (any (apple , freebsdlike))] impl UserDefinedFlags { # [doc = " Create a new `UserDefinedFlags` from a `u32`."] pub fn new (flags : u32) -> Self { Self (flags & EVFILT_USER_FLAGS) } # [doc = " Get the underlying `u32`."] pub fn get (self) -> u32 { self . 0 } }
};
}

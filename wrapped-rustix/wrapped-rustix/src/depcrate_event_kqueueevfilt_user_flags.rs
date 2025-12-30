// Generated macro for EVFILT_USER_FLAGS (const)
macro_rules! Depcrate_event_kqueueEVFILT_USER_FLAGS {
() => {
// Module: crate::event::kqueue
// Provides: {"EVFILT_USER_FLAGS"}
// Dependencies: {}
# [doc = " Bottom 24 bits of a `u32`."] # [cfg (any (apple , freebsdlike))] const EVFILT_USER_FLAGS : u32 = 0x00ff_ffff ;
};
}

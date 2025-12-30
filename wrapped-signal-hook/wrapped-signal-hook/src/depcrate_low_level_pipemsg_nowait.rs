// Generated macro for MSG_NOWAIT (const)
macro_rules! Depcrate_low_level_pipeMSG_NOWAIT {
() => {
// Module: crate::low_level::pipe
// Provides: {"MSG_NOWAIT"}
// Dependencies: {}
# [cfg (not (target_os = "aix"))] const MSG_NOWAIT : i32 = libc :: MSG_DONTWAIT ;
};
}

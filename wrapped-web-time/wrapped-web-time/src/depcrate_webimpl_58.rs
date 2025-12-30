// Generated macro for impl_58 (impl)
macro_rules! Depcrate_webimpl_58 {
() => {
// Module: crate::web
// Provides: {"impl_58"}
// Dependencies: {}
impl SystemTimeExt for SystemTime { fn to_std (self) -> std :: time :: SystemTime { StdSystemTime :: UNIX_EPOCH + self . 0 } fn from_std (time : std :: time :: SystemTime) -> SystemTime { Self :: UNIX_EPOCH + time . duration_since (StdSystemTime :: UNIX_EPOCH) . expect ("found `SystemTime` earlier than unix epoch") } }
};
}

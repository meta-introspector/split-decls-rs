// Generated macro for impl_583 (impl)
macro_rules! Depcrate_interop_utcdatetime_systemtimeimpl_583 {
() => {
// Module: crate::interop::utcdatetime_systemtime
// Provides: {"impl_583"}
// Dependencies: {}
impl From < SystemTime > for UtcDateTime { # [inline] fn from (system_time : SystemTime) -> Self { match system_time . duration_since (SystemTime :: UNIX_EPOCH) { Ok (duration) => Self :: UNIX_EPOCH + duration , Err (err) => Self :: UNIX_EPOCH - err . duration () , } } }
};
}

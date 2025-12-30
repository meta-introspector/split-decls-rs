// Generated macro for impl_558 (impl)
macro_rules! Depcrate_interop_offsetdatetime_systemtimeimpl_558 {
() => {
// Module: crate::interop::offsetdatetime_systemtime
// Provides: {"impl_558"}
// Dependencies: {}
impl From < SystemTime > for OffsetDateTime { # [inline] fn from (system_time : SystemTime) -> Self { match system_time . duration_since (SystemTime :: UNIX_EPOCH) { Ok (duration) => Self :: UNIX_EPOCH + duration , Err (err) => Self :: UNIX_EPOCH - err . duration () , } } }
};
}

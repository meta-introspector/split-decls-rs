// Generated macro for impl_559 (impl)
macro_rules! Depcrate_interop_offsetdatetime_systemtimeimpl_559 {
() => {
// Module: crate::interop::offsetdatetime_systemtime
// Provides: {"impl_559"}
// Dependencies: {}
impl From < OffsetDateTime > for SystemTime { # [inline] fn from (datetime : OffsetDateTime) -> Self { let duration = datetime - OffsetDateTime :: UNIX_EPOCH ; if duration . is_zero () { Self :: UNIX_EPOCH } else if duration . is_positive () { Self :: UNIX_EPOCH + duration . unsigned_abs () } else { debug_assert ! (duration . is_negative ()) ; Self :: UNIX_EPOCH - duration . unsigned_abs () } } }
};
}

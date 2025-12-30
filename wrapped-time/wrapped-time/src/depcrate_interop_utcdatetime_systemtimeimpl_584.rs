// Generated macro for impl_584 (impl)
macro_rules! Depcrate_interop_utcdatetime_systemtimeimpl_584 {
() => {
// Module: crate::interop::utcdatetime_systemtime
// Provides: {"impl_584"}
// Dependencies: {}
impl From < UtcDateTime > for SystemTime { # [inline] fn from (datetime : UtcDateTime) -> Self { let duration = datetime - UtcDateTime :: UNIX_EPOCH ; if duration . is_zero () { Self :: UNIX_EPOCH } else if duration . is_positive () { Self :: UNIX_EPOCH + duration . unsigned_abs () } else { debug_assert ! (duration . is_negative ()) ; Self :: UNIX_EPOCH - duration . unsigned_abs () } } }
};
}

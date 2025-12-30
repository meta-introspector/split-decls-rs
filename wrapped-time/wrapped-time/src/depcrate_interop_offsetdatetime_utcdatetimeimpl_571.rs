// Generated macro for impl_571 (impl)
macro_rules! Depcrate_interop_offsetdatetime_utcdatetimeimpl_571 {
() => {
// Module: crate::interop::offsetdatetime_utcdatetime
// Provides: {"impl_571"}
// Dependencies: {}
impl From < UtcDateTime > for OffsetDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn from (datetime : UtcDateTime) -> Self { datetime . as_primitive () . assume_utc () } }
};
}

// Generated macro for impl_570 (impl)
macro_rules! Depcrate_interop_offsetdatetime_utcdatetimeimpl_570 {
() => {
// Module: crate::interop::offsetdatetime_utcdatetime
// Provides: {"impl_570"}
// Dependencies: {}
impl From < OffsetDateTime > for UtcDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn from (datetime : OffsetDateTime) -> Self { datetime . to_utc () } }
};
}

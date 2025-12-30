// Generated macro for impl_565 (impl)
macro_rules! Depcrate_interop_offsetdatetime_utcdatetimeimpl_565 {
() => {
// Module: crate::interop::offsetdatetime_utcdatetime
// Provides: {"impl_565"}
// Dependencies: {}
impl Sub < UtcDateTime > for OffsetDateTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : UtcDateTime) -> Self :: Output { self - Self :: from (rhs) } }
};
}

// Generated macro for impl_564 (impl)
macro_rules! Depcrate_interop_offsetdatetime_utcdatetimeimpl_564 {
() => {
// Module: crate::interop::offsetdatetime_utcdatetime
// Provides: {"impl_564"}
// Dependencies: {}
impl Sub < OffsetDateTime > for UtcDateTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : OffsetDateTime) -> Self :: Output { OffsetDateTime :: from (self) - rhs } }
};
}

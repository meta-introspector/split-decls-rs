// Generated macro for impl_553 (impl)
macro_rules! Depcrate_interop_offsetdatetime_systemtimeimpl_553 {
() => {
// Module: crate::interop::offsetdatetime_systemtime
// Provides: {"impl_553"}
// Dependencies: {}
impl Sub < OffsetDateTime > for SystemTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] fn sub (self , rhs : OffsetDateTime) -> Self :: Output { OffsetDateTime :: from (self) - rhs } }
};
}

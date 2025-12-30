// Generated macro for impl_552 (impl)
macro_rules! Depcrate_interop_offsetdatetime_systemtimeimpl_552 {
() => {
// Module: crate::interop::offsetdatetime_systemtime
// Provides: {"impl_552"}
// Dependencies: {}
impl Sub < SystemTime > for OffsetDateTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] fn sub (self , rhs : SystemTime) -> Self :: Output { self - Self :: from (rhs) } }
};
}

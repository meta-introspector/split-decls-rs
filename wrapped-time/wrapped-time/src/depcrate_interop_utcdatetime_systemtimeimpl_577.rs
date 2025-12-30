// Generated macro for impl_577 (impl)
macro_rules! Depcrate_interop_utcdatetime_systemtimeimpl_577 {
() => {
// Module: crate::interop::utcdatetime_systemtime
// Provides: {"impl_577"}
// Dependencies: {}
impl Sub < SystemTime > for UtcDateTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : SystemTime) -> Self :: Output { self - Self :: from (rhs) } }
};
}

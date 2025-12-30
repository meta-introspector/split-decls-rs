// Generated macro for impl_578 (impl)
macro_rules! Depcrate_interop_utcdatetime_systemtimeimpl_578 {
() => {
// Module: crate::interop::utcdatetime_systemtime
// Provides: {"impl_578"}
// Dependencies: {}
impl Sub < UtcDateTime > for SystemTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : UtcDateTime) -> Self :: Output { UtcDateTime :: from (self) - rhs } }
};
}

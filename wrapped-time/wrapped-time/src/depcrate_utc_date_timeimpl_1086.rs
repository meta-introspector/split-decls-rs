// Generated macro for impl_1086 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1086 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1086"}
// Dependencies: {}
impl Sub for UtcDateTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : Self) -> Self :: Output { self . inner . sub (rhs . inner) } }
};
}

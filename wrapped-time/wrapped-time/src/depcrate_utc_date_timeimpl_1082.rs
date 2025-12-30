// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1082 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1082"}
// Dependencies: {}
impl Sub < Duration > for UtcDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : Duration) -> Self :: Output { self . checked_sub (rhs) . expect ("resulting value is out of range") } }
};
}

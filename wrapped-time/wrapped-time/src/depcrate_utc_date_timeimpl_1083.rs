// Generated macro for impl_1083 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1083 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1083"}
// Dependencies: {}
impl Sub < StdDuration > for UtcDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , duration : StdDuration) -> Self :: Output { Self :: from_primitive (self . inner . sub (duration)) } }
};
}

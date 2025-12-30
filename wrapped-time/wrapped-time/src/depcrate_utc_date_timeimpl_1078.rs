// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1078 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1078"}
// Dependencies: {}
impl Add < Duration > for UtcDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : Duration) -> Self :: Output { self . inner . add (duration) . as_utc () } }
};
}

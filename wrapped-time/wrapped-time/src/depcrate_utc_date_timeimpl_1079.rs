// Generated macro for impl_1079 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1079 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1079"}
// Dependencies: {}
impl Add < StdDuration > for UtcDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : StdDuration) -> Self :: Output { self . inner . add (duration) . as_utc () } }
};
}

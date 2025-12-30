// Generated macro for impl_1081 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1081 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1081"}
// Dependencies: {}
impl AddAssign < StdDuration > for UtcDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add_assign (& mut self , rhs : StdDuration) { self . inner . add_assign (rhs) ; } }
};
}

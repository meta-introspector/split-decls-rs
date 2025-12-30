// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1080 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1080"}
// Dependencies: {}
impl AddAssign < Duration > for UtcDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add_assign (& mut self , rhs : Duration) { self . inner . add_assign (rhs) ; } }
};
}

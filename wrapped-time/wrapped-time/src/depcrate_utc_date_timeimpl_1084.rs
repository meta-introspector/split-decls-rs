// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1084 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1084"}
// Dependencies: {}
impl SubAssign < Duration > for UtcDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub_assign (& mut self , rhs : Duration) { self . inner . sub_assign (rhs) ; } }
};
}

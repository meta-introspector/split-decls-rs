// Generated macro for impl_1085 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1085 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1085"}
// Dependencies: {}
impl SubAssign < StdDuration > for UtcDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub_assign (& mut self , rhs : StdDuration) { self . inner . sub_assign (rhs) ; } }
};
}

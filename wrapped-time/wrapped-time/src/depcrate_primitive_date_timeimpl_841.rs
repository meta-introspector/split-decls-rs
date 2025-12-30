// Generated macro for impl_841 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_841 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_841"}
// Dependencies: {}
impl AddAssign < Duration > for PrimitiveDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add_assign (& mut self , duration : Duration) { * self = * self + duration ; } }
};
}

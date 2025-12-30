// Generated macro for impl_842 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_842 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_842"}
// Dependencies: {}
impl AddAssign < StdDuration > for PrimitiveDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add_assign (& mut self , duration : StdDuration) { * self = * self + duration ; } }
};
}

// Generated macro for impl_846 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_846 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_846"}
// Dependencies: {}
impl SubAssign < StdDuration > for PrimitiveDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub_assign (& mut self , duration : StdDuration) { * self = * self - duration ; } }
};
}

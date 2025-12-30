// Generated macro for impl_845 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_845 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_845"}
// Dependencies: {}
impl SubAssign < Duration > for PrimitiveDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub_assign (& mut self , duration : Duration) { * self = * self - duration ; } }
};
}

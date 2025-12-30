// Generated macro for impl_647 (impl)
macro_rules! Depcrate_offset_date_timeimpl_647 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_647"}
// Dependencies: {}
impl SubAssign < StdDuration > for OffsetDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub_assign (& mut self , rhs : StdDuration) { * self = * self - rhs ; } }
};
}

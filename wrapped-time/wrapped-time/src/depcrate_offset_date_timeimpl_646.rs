// Generated macro for impl_646 (impl)
macro_rules! Depcrate_offset_date_timeimpl_646 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_646"}
// Dependencies: {}
impl SubAssign < Duration > for OffsetDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub_assign (& mut self , rhs : Duration) { * self = * self - rhs ; } }
};
}

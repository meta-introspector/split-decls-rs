// Generated macro for impl_642 (impl)
macro_rules! Depcrate_offset_date_timeimpl_642 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_642"}
// Dependencies: {}
impl AddAssign < Duration > for OffsetDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add_assign (& mut self , rhs : Duration) { * self = * self + rhs ; } }
};
}

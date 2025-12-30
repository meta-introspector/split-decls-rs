// Generated macro for impl_643 (impl)
macro_rules! Depcrate_offset_date_timeimpl_643 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_643"}
// Dependencies: {}
impl AddAssign < StdDuration > for OffsetDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add_assign (& mut self , rhs : StdDuration) { * self = * self + rhs ; } }
};
}

// Generated macro for impl_648 (impl)
macro_rules! Depcrate_offset_date_timeimpl_648 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_648"}
// Dependencies: {}
impl Sub for OffsetDateTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : Self) -> Self :: Output { let base = self . date_time () - rhs . date_time () ; let adjustment = Duration :: seconds ((self . offset . whole_seconds () - rhs . offset . whole_seconds ()) . extend :: < i64 > () ,) ; base - adjustment } }
};
}

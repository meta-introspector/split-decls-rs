// Generated macro for impl_644 (impl)
macro_rules! Depcrate_offset_date_timeimpl_644 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_644"}
// Dependencies: {}
impl Sub < Duration > for OffsetDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : Duration) -> Self :: Output { self . checked_sub (rhs) . expect ("resulting value is out of range") } }
};
}

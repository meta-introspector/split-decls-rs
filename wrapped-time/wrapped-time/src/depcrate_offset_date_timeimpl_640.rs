// Generated macro for impl_640 (impl)
macro_rules! Depcrate_offset_date_timeimpl_640 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_640"}
// Dependencies: {}
impl Add < Duration > for OffsetDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : Duration) -> Self :: Output { self . checked_add (duration) . expect ("resulting value is out of range") } }
};
}

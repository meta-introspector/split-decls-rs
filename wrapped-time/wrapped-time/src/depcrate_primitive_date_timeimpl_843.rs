// Generated macro for impl_843 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_843 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_843"}
// Dependencies: {}
impl Sub < Duration > for PrimitiveDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , duration : Duration) -> Self :: Output { self . checked_sub (duration) . expect ("resulting value is out of range") } }
};
}

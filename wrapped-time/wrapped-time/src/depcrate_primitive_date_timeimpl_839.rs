// Generated macro for impl_839 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_839 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_839"}
// Dependencies: {}
impl Add < Duration > for PrimitiveDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : Duration) -> Self :: Output { self . checked_add (duration) . expect ("resulting value is out of range") } }
};
}

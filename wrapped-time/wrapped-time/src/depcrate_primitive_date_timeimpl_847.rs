// Generated macro for impl_847 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_847 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_847"}
// Dependencies: {}
impl Sub for PrimitiveDateTime { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : Self) -> Self :: Output { (self . date - rhs . date) + (self . time - rhs . time) } }
};
}

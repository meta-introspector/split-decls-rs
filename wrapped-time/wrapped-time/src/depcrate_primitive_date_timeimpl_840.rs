// Generated macro for impl_840 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_840 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_840"}
// Dependencies: {}
impl Add < StdDuration > for PrimitiveDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : StdDuration) -> Self :: Output { let (is_next_day , time) = self . time . adjusting_add_std (duration) ; Self { date : if is_next_day { (self . date + duration) . next_day () . expect ("resulting value is out of range") } else { self . date + duration } , time , } } }
};
}

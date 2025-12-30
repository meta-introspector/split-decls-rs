// Generated macro for impl_844 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_844 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_844"}
// Dependencies: {}
impl Sub < StdDuration > for PrimitiveDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , duration : StdDuration) -> Self :: Output { let (is_previous_day , time) = self . time . adjusting_sub_std (duration) ; Self { date : if is_previous_day { (self . date - duration) . previous_day () . expect ("resulting value is out of range") } else { self . date - duration } , time , } } }
};
}

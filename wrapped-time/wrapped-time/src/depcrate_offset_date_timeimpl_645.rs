// Generated macro for impl_645 (impl)
macro_rules! Depcrate_offset_date_timeimpl_645 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_645"}
// Dependencies: {}
impl Sub < StdDuration > for OffsetDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , duration : StdDuration) -> Self :: Output { let (is_previous_day , time) = self . time () . adjusting_sub_std (duration) ; Self :: new_in_offset (if is_previous_day { (self . date () - duration) . previous_day () . expect ("resulting value is out of range") } else { self . date () - duration } , time , self . offset ,) } }
};
}

// Generated macro for impl_641 (impl)
macro_rules! Depcrate_offset_date_timeimpl_641 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_641"}
// Dependencies: {}
impl Add < StdDuration > for OffsetDateTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : StdDuration) -> Self :: Output { let (is_next_day , time) = self . time () . adjusting_add_std (duration) ; Self :: new_in_offset (if is_next_day { (self . date () + duration) . next_day () . expect ("resulting value is out of range") } else { self . date () + duration } , time , self . offset ,) } }
};
}

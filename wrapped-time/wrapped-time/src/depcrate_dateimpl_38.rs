// Generated macro for impl_38 (impl)
macro_rules! Depcrate_dateimpl_38 {
() => {
// Module: crate::date
// Provides: {"impl_38"}
// Dependencies: {}
impl Sub < StdDuration > for Date { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , duration : StdDuration) -> Self :: Output { self . checked_sub_std (duration) . expect ("overflow subtracting duration from date") } }
};
}

// Generated macro for impl_37 (impl)
macro_rules! Depcrate_dateimpl_37 {
() => {
// Module: crate::date
// Provides: {"impl_37"}
// Dependencies: {}
impl Sub < Duration > for Date { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , duration : Duration) -> Self :: Output { self . checked_sub (duration) . expect ("overflow subtracting duration from date") } }
};
}

// Generated macro for impl_35 (impl)
macro_rules! Depcrate_dateimpl_35 {
() => {
// Module: crate::date
// Provides: {"impl_35"}
// Dependencies: {}
impl Add < StdDuration > for Date { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : StdDuration) -> Self :: Output { self . checked_add_std (duration) . expect ("overflow adding duration to date") } }
};
}

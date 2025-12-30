// Generated macro for impl_34 (impl)
macro_rules! Depcrate_dateimpl_34 {
() => {
// Module: crate::date
// Provides: {"impl_34"}
// Dependencies: {}
impl Add < Duration > for Date { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , duration : Duration) -> Self :: Output { self . checked_add (duration) . expect ("overflow adding duration to date") } }
};
}

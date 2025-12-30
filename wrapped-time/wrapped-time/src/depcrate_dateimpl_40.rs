// Generated macro for impl_40 (impl)
macro_rules! Depcrate_dateimpl_40 {
() => {
// Module: crate::date
// Provides: {"impl_40"}
// Dependencies: {}
impl Sub for Date { type Output = Duration ; # [inline] fn sub (self , other : Self) -> Self :: Output { Duration :: days ((self . to_julian_day () - other . to_julian_day ()) . extend ()) } }
};
}

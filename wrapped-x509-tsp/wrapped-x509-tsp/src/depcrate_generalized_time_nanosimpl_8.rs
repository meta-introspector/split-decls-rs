// Generated macro for impl_8 (impl)
macro_rules! Depcrate_generalized_time_nanosimpl_8 {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"impl_8"}
// Dependencies: {}
impl From < GeneralizedTime > for GeneralizedTimeNanos { fn from (time : GeneralizedTime) -> Self { Self { datetime : time . to_date_time () , nanoseconds : 0 , } } }
};
}

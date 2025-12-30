// Generated macro for impl_77 (impl)
macro_rules! Depcrate_timeimpl_77 {
() => {
// Module: crate::time
// Provides: {"impl_77"}
// Dependencies: {}
impl From < UtcTime > for OcspGeneralizedTime { fn from (other : UtcTime) -> Self { Self (GeneralizedTime :: from_date_time (other . to_date_time ())) } }
};
}

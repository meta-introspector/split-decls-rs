// Generated macro for impl_78 (impl)
macro_rules! Depcrate_timeimpl_78 {
() => {
// Module: crate::time
// Provides: {"impl_78"}
// Dependencies: {}
impl From < Time > for OcspGeneralizedTime { fn from (other : Time) -> Self { match other { Time :: UtcTime (t) => t . into () , Time :: GeneralTime (t) => t . into () , } } }
};
}

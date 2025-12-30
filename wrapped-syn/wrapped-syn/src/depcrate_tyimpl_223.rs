// Generated macro for impl_223 (impl)
macro_rules! Depcrate_tyimpl_223 {
() => {
// Module: crate::ty
// Provides: {"impl_223"}
// Dependencies: {}
impl < T > From < T > for Path where T : Into < PathSegment > { fn from (segment : T) -> Self { Path { leading_colon : None , segments : vec ! [(segment . into () , None)] . into () , } } }
};
}

// Generated macro for impl_225 (impl)
macro_rules! Depcrate_tyimpl_225 {
() => {
// Module: crate::ty
// Provides: {"impl_225"}
// Dependencies: {}
impl < T > From < T > for PathSegment where T : Into < Ident > { fn from (ident : T) -> Self { PathSegment { ident : ident . into () , parameters : PathParameters :: None , } } }
};
}

// Generated macro for impl_813 (impl)
macro_rules! Depcrate_pathimpl_813 {
() => {
// Module: crate::path
// Provides: {"impl_813"}
// Dependencies: {}
impl < T > From < T > for PathSegment where T : Into < Ident > , { fn from (ident : T) -> Self { PathSegment { ident : ident . into () , arguments : PathArguments :: None , } } }
};
}

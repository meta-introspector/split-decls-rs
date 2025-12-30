// Generated macro for impl_829 (impl)
macro_rules! Depcrate_pathimpl_829 {
() => {
// Module: crate::path
// Provides: {"impl_829"}
// Dependencies: {}
impl < T > From < T > for PathSegment where T : Into < Ident > , { fn from (ident : T) -> Self { PathSegment { ident : ident . into () , arguments : PathArguments :: None , } } }
};
}

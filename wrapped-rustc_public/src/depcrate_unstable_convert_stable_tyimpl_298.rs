// Generated macro for impl_298 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_298 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_298"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: BoundTy { type T = crate :: ty :: BoundTy ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: BoundTy ; BoundTy { var : self . var . as_usize () , kind : self . kind . stable (tables , cx) } } }
};
}

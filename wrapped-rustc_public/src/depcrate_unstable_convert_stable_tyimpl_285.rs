// Generated macro for impl_285 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_285 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_285"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: FnSig < 'tcx > { type T = crate :: ty :: FnSig ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: FnSig ; FnSig { inputs_and_output : self . inputs_and_output . iter () . map (| ty | ty . stable (tables , cx)) . collect () , c_variadic : self . c_variadic , safety : self . safety . stable (tables , cx) , abi : self . abi . stable (tables , cx) , } } }
};
}

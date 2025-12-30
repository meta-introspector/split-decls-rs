// Generated macro for impl_327 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_327 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_middle :: ty :: util :: Discr < 'tcx > { type T = crate :: ty :: Discr ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: Discr { val : self . val , ty : self . ty . stable (tables , cx) } } }
};
}

// Generated macro for impl_190 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_190 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: LayoutData < rustc_abi :: FieldIdx , rustc_abi :: VariantIdx > { type T = LayoutShape ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { LayoutShape { fields : self . fields . stable (tables , cx) , variants : self . variants . stable (tables , cx) , abi : self . backend_repr . stable (tables , cx) , abi_align : self . align . abi . stable (tables , cx) , size : self . size . stable (tables , cx) , } } }
};
}

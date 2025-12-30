// Generated macro for impl_188 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_188 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: TyAndLayout < 'tcx , ty :: Ty < 'tcx > > { type T = TyAndLayout ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { TyAndLayout { ty : self . ty . stable (tables , cx) , layout : self . layout . stable (tables , cx) } } }
};
}

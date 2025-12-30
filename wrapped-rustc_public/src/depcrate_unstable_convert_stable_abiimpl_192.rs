// Generated macro for impl_192 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_192 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_192"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for callconv :: ArgAbi < 'tcx , ty :: Ty < 'tcx > > { type T = ArgAbi ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { ArgAbi { ty : self . layout . ty . stable (tables , cx) , layout : self . layout . layout . stable (tables , cx) , mode : self . mode . stable (tables , cx) , } } }
};
}

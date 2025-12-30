// Generated macro for impl_191 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_191 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for callconv :: FnAbi < 'tcx , ty :: Ty < 'tcx > > { type T = FnAbi ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { assert ! (self . args . len () >= self . fixed_count as usize) ; assert ! (! self . c_variadic || matches ! (self . conv , CanonAbi :: C)) ; FnAbi { args : self . args . as_ref () . stable (tables , cx) , ret : self . ret . stable (tables , cx) , fixed_count : self . fixed_count , conv : self . conv . stable (tables , cx) , c_variadic : self . c_variadic , } } }
};
}

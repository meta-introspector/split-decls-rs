// Generated macro for impl_27 (impl)
macro_rules! Depcrate_exportimpl_27 {
() => {
// Module: crate::export
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'tcx > AbiHashStable < 'tcx > for ty :: GenericArgKind < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { match self { ty :: GenericArgKind :: Type (t) => t . abi_hash (tcx , hasher) , ty :: GenericArgKind :: Lifetime (_) | ty :: GenericArgKind :: Const (_) => unimplemented ! () , } } }
};
}

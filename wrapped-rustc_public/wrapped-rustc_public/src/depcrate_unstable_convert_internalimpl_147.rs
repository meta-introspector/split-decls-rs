// Generated macro for impl_147 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_147 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_147"}
// Dependencies: {}
impl RustcInternal for BoundVariableKind { type T < 'tcx > = rustc_ty :: BoundVariableKind ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { BoundVariableKind :: Ty (kind) => rustc_ty :: BoundVariableKind :: Ty (match kind { BoundTyKind :: Anon => rustc_ty :: BoundTyKind :: Anon , BoundTyKind :: Param (def , _symbol) => { rustc_ty :: BoundTyKind :: Param (def . 0 . internal (tables , tcx)) } }) , BoundVariableKind :: Region (kind) => rustc_ty :: BoundVariableKind :: Region (match kind { BoundRegionKind :: BrAnon => rustc_ty :: BoundRegionKind :: Anon , BoundRegionKind :: BrNamed (def , _symbol) => { rustc_ty :: BoundRegionKind :: Named (def . 0 . internal (tables , tcx)) } BoundRegionKind :: BrEnv => rustc_ty :: BoundRegionKind :: ClosureEnv , }) , BoundVariableKind :: Const => rustc_ty :: BoundVariableKind :: Const , } } }
};
}

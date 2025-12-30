// Generated macro for impl_318 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_318 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_318"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: Instance < 'tcx > { type T = crate :: mir :: mono :: Instance ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let def = tables . instance_def (cx . lift (* self) . unwrap ()) ; let kind = match self . def { ty :: InstanceKind :: Item (..) => crate :: mir :: mono :: InstanceKind :: Item , ty :: InstanceKind :: Intrinsic (..) => crate :: mir :: mono :: InstanceKind :: Intrinsic , ty :: InstanceKind :: Virtual (_def_id , idx) => { crate :: mir :: mono :: InstanceKind :: Virtual { idx } } ty :: InstanceKind :: VTableShim (..) | ty :: InstanceKind :: ReifyShim (..) | ty :: InstanceKind :: FnPtrAddrShim (..) | ty :: InstanceKind :: ClosureOnceShim { .. } | ty :: InstanceKind :: ConstructCoroutineInClosureShim { .. } | ty :: InstanceKind :: ThreadLocalShim (..) | ty :: InstanceKind :: DropGlue (..) | ty :: InstanceKind :: CloneShim (..) | ty :: InstanceKind :: FnPtrShim (..) | ty :: InstanceKind :: FutureDropPollShim (..) | ty :: InstanceKind :: AsyncDropGlue (..) | ty :: InstanceKind :: AsyncDropGlueCtorShim (..) => crate :: mir :: mono :: InstanceKind :: Shim , } ; crate :: mir :: mono :: Instance { def , kind } } }
};
}

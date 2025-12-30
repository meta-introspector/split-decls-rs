// Generated macro for impl_303 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_303 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_303"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_middle :: ty :: GenericParamDefKind { type T = crate :: ty :: GenericParamDefKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: GenericParamDefKind ; match * self { ty :: GenericParamDefKind :: Lifetime => GenericParamDefKind :: Lifetime , ty :: GenericParamDefKind :: Type { has_default , synthetic } => { GenericParamDefKind :: Type { has_default , synthetic } } ty :: GenericParamDefKind :: Const { has_default } => { GenericParamDefKind :: Const { has_default } } } } }
};
}

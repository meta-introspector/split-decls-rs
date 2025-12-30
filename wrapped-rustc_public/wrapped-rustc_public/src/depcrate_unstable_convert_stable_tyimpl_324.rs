// Generated macro for impl_324 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_324 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_324"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: AssocContainer { type T = crate :: ty :: AssocContainer ; fn stable (& self , tables : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys > ,) -> Self :: T { use crate :: ty :: AssocContainer ; match self { ty :: AssocContainer :: Trait => AssocContainer :: Trait , ty :: AssocContainer :: InherentImpl => AssocContainer :: InherentImpl , ty :: AssocContainer :: TraitImpl (trait_item_id) => { AssocContainer :: TraitImpl (tables . assoc_def (trait_item_id . unwrap ())) } } } }
};
}

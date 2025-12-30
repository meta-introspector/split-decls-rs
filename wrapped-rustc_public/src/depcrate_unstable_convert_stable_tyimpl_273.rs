// Generated macro for impl_273 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_273 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ExistentialPredicate < 'tcx > { type T = crate :: ty :: ExistentialPredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: ExistentialPredicate :: * ; match self { ty :: ExistentialPredicate :: Trait (existential_trait_ref) => { Trait (existential_trait_ref . stable (tables , cx)) } ty :: ExistentialPredicate :: Projection (existential_projection) => { Projection (existential_projection . stable (tables , cx)) } ty :: ExistentialPredicate :: AutoTrait (def_id) => AutoTrait (tables . trait_def (* def_id)) , } } }
};
}

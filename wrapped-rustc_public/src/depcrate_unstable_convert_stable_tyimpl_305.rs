// Generated macro for impl_305 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_305 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_305"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: PredicateKind < 'tcx > { type T = crate :: ty :: PredicateKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: ty :: PredicateKind ; match self { PredicateKind :: Clause (clause_kind) => { crate :: ty :: PredicateKind :: Clause (clause_kind . stable (tables , cx)) } PredicateKind :: DynCompatible (did) => { crate :: ty :: PredicateKind :: DynCompatible (tables . trait_def (* did)) } PredicateKind :: Subtype (subtype_predicate) => { crate :: ty :: PredicateKind :: SubType (subtype_predicate . stable (tables , cx)) } PredicateKind :: Coerce (coerce_predicate) => { crate :: ty :: PredicateKind :: Coerce (coerce_predicate . stable (tables , cx)) } PredicateKind :: ConstEquate (a , b) => { crate :: ty :: PredicateKind :: ConstEquate (a . stable (tables , cx) , b . stable (tables , cx)) } PredicateKind :: Ambiguous => crate :: ty :: PredicateKind :: Ambiguous , PredicateKind :: NormalizesTo (_pred) => unimplemented ! () , PredicateKind :: AliasRelate (a , b , alias_relation_direction) => { crate :: ty :: PredicateKind :: AliasRelate (a . kind () . stable (tables , cx) , b . kind () . stable (tables , cx) , alias_relation_direction . stable (tables , cx) ,) } } } }
};
}

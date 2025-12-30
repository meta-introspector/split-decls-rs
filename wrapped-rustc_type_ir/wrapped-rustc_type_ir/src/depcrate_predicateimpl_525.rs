// Generated macro for impl_525 (impl)
macro_rules! Depcrate_predicateimpl_525 {
() => {
// Module: crate::predicate
// Provides: {"impl_525"}
// Dependencies: {}
impl < I : Interner > ty :: Binder < I , TraitRef < I > > { pub fn self_ty (& self) -> ty :: Binder < I , I :: Ty > { self . map_bound_ref (| tr | tr . self_ty ()) } pub fn def_id (& self) -> I :: TraitId { self . skip_binder () . def_id } pub fn to_host_effect_clause (self , cx : I , constness : BoundConstness) -> I :: Clause { self . map_bound (| trait_ref | { ty :: ClauseKind :: HostEffect (HostEffectPredicate { trait_ref , constness }) }) . upcast (cx) } }
};
}

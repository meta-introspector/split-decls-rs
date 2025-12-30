// Generated macro for impl_572 (impl)
macro_rules! Depcrate_predicateimpl_572 {
() => {
// Module: crate::predicate
// Provides: {"impl_572"}
// Dependencies: {}
impl < I : Interner > ty :: Binder < I , HostEffectPredicate < I > > { pub fn def_id (self) -> I :: TraitId { self . skip_binder () . def_id () } pub fn self_ty (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| trait_ref | trait_ref . self_ty ()) } # [inline] pub fn constness (self) -> BoundConstness { self . skip_binder () . constness } }
};
}

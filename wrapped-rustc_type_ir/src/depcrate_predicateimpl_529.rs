// Generated macro for impl_529 (impl)
macro_rules! Depcrate_predicateimpl_529 {
() => {
// Module: crate::predicate
// Provides: {"impl_529"}
// Dependencies: {}
impl < I : Interner > ty :: Binder < I , TraitPredicate < I > > { pub fn def_id (self) -> I :: TraitId { self . skip_binder () . def_id () } pub fn self_ty (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| trait_ref | trait_ref . self_ty ()) } # [inline] pub fn polarity (self) -> PredicatePolarity { self . skip_binder () . polarity } }
};
}

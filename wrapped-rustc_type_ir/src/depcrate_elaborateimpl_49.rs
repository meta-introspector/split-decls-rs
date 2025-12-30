// Generated macro for impl_49 (impl)
macro_rules! Depcrate_elaborateimpl_49 {
() => {
// Module: crate::elaborate
// Provides: {"impl_49"}
// Dependencies: {}
impl < I : Interner , It : Iterator < Item = I :: Clause > > Iterator for FilterToTraits < I , It > { type Item = ty :: Binder < I , ty :: TraitRef < I > > ; fn next (& mut self) -> Option < ty :: Binder < I , ty :: TraitRef < I > > > { while let Some (pred) = self . base_iterator . next () { if let Some (data) = pred . as_trait_clause () { return Some (data . map_bound (| t | t . trait_ref)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . base_iterator . size_hint () ; (0 , upper) } }
};
}

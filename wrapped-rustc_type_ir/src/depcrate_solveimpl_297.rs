// Generated macro for impl_297 (impl)
macro_rules! Depcrate_solveimpl_297 {
() => {
// Module: crate::solve
// Provides: {"impl_297"}
// Dependencies: {}
impl SizedTraitKind { # [doc = " Returns `DefId` of corresponding language item."] pub fn require_lang_item < I : Interner > (self , cx : I) -> I :: TraitId { cx . require_trait_lang_item (match self { SizedTraitKind :: Sized => SolverTraitLangItem :: Sized , SizedTraitKind :: MetaSized => SolverTraitLangItem :: MetaSized , }) } }
};
}

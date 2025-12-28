macro_rules! deps {
    () => {
        SizedTraitKind!();
        SolverTraitLangItem!();
        DefId!();
        Interner!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl SizedTraitKind { # [doc = " Returns `DefId` of corresponding language item."] pub fn require_lang_item < I : Interner > (self , cx : I) -> I :: TraitId { cx . require_trait_lang_item (match self { SizedTraitKind :: Sized => SolverTraitLangItem :: Sized , SizedTraitKind :: MetaSized => SolverTraitLangItem :: MetaSized , }) } }
    };
}

impl_204!();
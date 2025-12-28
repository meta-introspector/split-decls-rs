macro_rules! deps {
    () => {
        BoundVarReplacer!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a , Infcx , I > BoundVarReplacer < 'a , Infcx , I > where Infcx : InferCtxtLike < Interner = I > , I : Interner , { # [doc = " Returns `Some` if we *were* able to replace bound vars. If there are any bound vars that"] # [doc = " use a binding level above `universe_indices.len()`, we fail."] pub fn replace_bound_vars < T : TypeFoldable < I > > (infcx : & 'a Infcx , universe_indices : & 'a mut Vec < Option < ty :: UniverseIndex > > , value : T ,) -> (T , IndexMap < I :: PlaceholderRegion , I :: BoundRegion > , IndexMap < I :: PlaceholderTy , I :: BoundTy > , IndexMap < I :: PlaceholderConst , I :: BoundConst > ,) { let mut replacer = BoundVarReplacer { infcx , mapped_regions : Default :: default () , mapped_types : Default :: default () , mapped_consts : Default :: default () , current_index : ty :: INNERMOST , universe_indices , } ; let value = value . fold_with (& mut replacer) ; (value , replacer . mapped_regions , replacer . mapped_types , replacer . mapped_consts) } fn universe_for (& mut self , debruijn : ty :: DebruijnIndex) -> ty :: UniverseIndex { let infcx = self . infcx ; let index = self . universe_indices . len () + self . current_index . as_usize () - debruijn . as_usize () - 1 ; let universe = self . universe_indices [index] . unwrap_or_else (| | { for i in self . universe_indices . iter_mut () . take (index + 1) { * i = i . or_else (| | Some (infcx . create_next_universe ())) } self . universe_indices [index] . unwrap () }) ; universe } }
    };
}

impl_26!()
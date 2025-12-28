macro_rules! BoundVarReplacer {
    () => {
        pub struct BoundVarReplacer < 'a , Infcx , I = < Infcx as InferCtxtLike > :: Interner > where Infcx : InferCtxtLike < Interner = I > , I : Interner , { infcx : & 'a Infcx , mapped_regions : IndexMap < I :: PlaceholderRegion , I :: BoundRegion > , mapped_types : IndexMap < I :: PlaceholderTy , I :: BoundTy > , mapped_consts : IndexMap < I :: PlaceholderConst , I :: BoundConst > , current_index : ty :: DebruijnIndex , universe_indices : & 'a mut Vec < Option < ty :: UniverseIndex > > , }
    };
}

BoundVarReplacer!();
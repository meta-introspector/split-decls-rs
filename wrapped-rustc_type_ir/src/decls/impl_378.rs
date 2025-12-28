macro_rules! deps {
    () => {
        AliasTermKind!();
        AliasTerm!();
        Interner!();
        GenericArgs!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        # [doc = " The following methods work only with inherent associated term projections."] impl < I : Interner > AliasTerm < I > { # [doc = " Transform the generic parameters to have the given `impl` args as the base and the GAT args on top of that."] # [doc = ""] # [doc = " Does the following transformation:"] # [doc = ""] # [doc = " ```text"] # [doc = " [Self, P_0...P_m] -> [I_0...I_n, P_0...P_m]"] # [doc = ""] # [doc = "     I_i impl args"] # [doc = "     P_j GAT args"] # [doc = " ```"] pub fn rebase_inherent_args_onto_impl (self , impl_args : I :: GenericArgs , interner : I ,) -> I :: GenericArgs { debug_assert ! (matches ! (self . kind (interner) , AliasTermKind :: InherentTy | AliasTermKind :: InherentConst)) ; interner . mk_args_from_iter (impl_args . iter () . chain (self . args . iter () . skip (1))) } }
    };
}

impl_378!()
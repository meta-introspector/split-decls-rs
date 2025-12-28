macro_rules! deps {
    () => {
        EarlyBinder!();
        TraitRef!();
    };
}

macro_rules! ImplTrait {
    () => {
        deps!();
        pub type ImplTrait = EarlyBinder < TraitRef > ;
    };
}

ImplTrait!()
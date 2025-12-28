macro_rules! deps {
    () => {
        TraitRef!();
        PatternKind!();
        ExistentialTraitRef!();
    };
}

macro_rules! macro_90 {
    () => {
        deps!();
        define_debug_via_print ! (TraitRef , ExistentialTraitRef , PatternKind) ;
    };
}

macro_90!();
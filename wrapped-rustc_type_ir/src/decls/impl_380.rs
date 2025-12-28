macro_rules! deps {
    () => {
        UnevaluatedConst!();
        Interner!();
        AliasTerm!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < I : Interner > From < ty :: UnevaluatedConst < I > > for AliasTerm < I > { fn from (ct : ty :: UnevaluatedConst < I >) -> Self { AliasTerm { args : ct . args , def_id : ct . def , _use_alias_term_new_instead : () } } }
    };
}

impl_380!();
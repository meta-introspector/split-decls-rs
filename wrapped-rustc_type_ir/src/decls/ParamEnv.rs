macro_rules! deps {
    () => {
        Clause!();
        SliceLike!();
        Interner!();
        TypeFoldable!();
    };
}

macro_rules! ParamEnv {
    () => {
        deps!();
        pub trait ParamEnv < I : Interner > : Copy + Debug + Hash + Eq + TypeFoldable < I > { fn caller_bounds (self) -> impl SliceLike < Item = I :: Clause > ; }
    };
}

ParamEnv!();
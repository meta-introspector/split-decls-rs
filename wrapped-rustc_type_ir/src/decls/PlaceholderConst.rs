macro_rules! deps {
    () => {
        Interner!();
        PlaceholderLike!();
        ParamEnv!();
        Ty!();
    };
}

macro_rules! PlaceholderConst {
    () => {
        deps!();
        pub trait PlaceholderConst < I : Interner > : PlaceholderLike < I , Bound = I :: BoundConst > { fn find_const_ty_from_env (self , env : I :: ParamEnv) -> I :: Ty ; }
    };
}

PlaceholderConst!()
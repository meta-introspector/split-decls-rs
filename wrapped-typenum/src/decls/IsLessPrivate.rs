macro_rules! deps {
    () => {
        Bit!();
        Cmp!();
    };
}

macro_rules! IsLessPrivate {
    () => {
        deps!();
        pub trait IsLessPrivate < Rhs , Cmp > { type Output : Bit ; # [allow (clippy :: wrong_self_convention)] fn is_less_private (self , _ : Rhs , _ : Cmp) -> Self :: Output ; }
    };
}

IsLessPrivate!();
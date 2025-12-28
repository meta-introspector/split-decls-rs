macro_rules! deps {
    () => {
        Cmp!();
        Bit!();
    };
}

macro_rules! IsLessOrEqualPrivate {
    () => {
        deps!();
        pub trait IsLessOrEqualPrivate < Rhs , Cmp > { type Output : Bit ; # [allow (clippy :: wrong_self_convention)] fn is_less_or_equal_private (self , _ : Rhs , _ : Cmp) -> Self :: Output ; }
    };
}

IsLessOrEqualPrivate!();
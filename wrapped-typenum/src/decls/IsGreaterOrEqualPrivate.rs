macro_rules! deps {
    () => {
        Cmp!();
        Bit!();
    };
}

macro_rules! IsGreaterOrEqualPrivate {
    () => {
        deps!();
        pub trait IsGreaterOrEqualPrivate < Rhs , Cmp > { type Output : Bit ; # [allow (clippy :: wrong_self_convention)] fn is_greater_or_equal_private (self , _ : Rhs , _ : Cmp) -> Self :: Output ; }
    };
}

IsGreaterOrEqualPrivate!();
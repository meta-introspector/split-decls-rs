macro_rules! deps {
    () => {
        Cmp!();
        Bit!();
    };
}

macro_rules! IsNotEqualPrivate {
    () => {
        deps!();
        pub trait IsNotEqualPrivate < Rhs , Cmp > { type Output : Bit ; # [allow (clippy :: wrong_self_convention)] fn is_not_equal_private (self , _ : Rhs , _ : Cmp) -> Self :: Output ; }
    };
}

IsNotEqualPrivate!();
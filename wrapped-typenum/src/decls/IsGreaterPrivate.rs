macro_rules! deps {
    () => {
        Bit!();
        Cmp!();
    };
}

macro_rules! IsGreaterPrivate {
    () => {
        deps!();
        pub trait IsGreaterPrivate < Rhs , Cmp > { type Output : Bit ; # [allow (clippy :: wrong_self_convention)] fn is_greater_private (self , _ : Rhs , _ : Cmp) -> Self :: Output ; }
    };
}

IsGreaterPrivate!();
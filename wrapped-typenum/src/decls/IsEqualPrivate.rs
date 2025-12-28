macro_rules! deps {
    () => {
        Cmp!();
        Bit!();
    };
}

macro_rules! IsEqualPrivate {
    () => {
        deps!();
        pub trait IsEqualPrivate < Rhs , Cmp > { type Output : Bit ; # [allow (clippy :: wrong_self_convention)] fn is_equal_private (self , _ : Rhs , _ : Cmp) -> Self :: Output ; }
    };
}

IsEqualPrivate!()
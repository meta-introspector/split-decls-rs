macro_rules! PrivateCmp {
    () => {
        pub trait PrivateCmp < Rhs , SoFar > { type Output ; fn private_cmp (& self , _ : & Rhs , _ : SoFar) -> Self :: Output ; }
    };
}

PrivateCmp!()
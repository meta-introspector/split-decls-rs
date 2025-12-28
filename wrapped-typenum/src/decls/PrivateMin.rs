macro_rules! PrivateMin {
    () => {
        pub trait PrivateMin < Rhs , CmpResult > { type Output ; fn private_min (self , rhs : Rhs) -> Self :: Output ; }
    };
}

PrivateMin!()
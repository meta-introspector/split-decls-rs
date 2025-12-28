macro_rules! PrivateMax {
    () => {
        pub trait PrivateMax < Rhs , CmpResult > { type Output ; fn private_max (self , rhs : Rhs) -> Self :: Output ; }
    };
}

PrivateMax!();
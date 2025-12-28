macro_rules! PrivateRem {
    () => {
        pub trait PrivateRem < URem , Divisor > { type Output ; fn private_rem (self , _ : URem , _ : Divisor) -> Self :: Output ; }
    };
}

PrivateRem!()
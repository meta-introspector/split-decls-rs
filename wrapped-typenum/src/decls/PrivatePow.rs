macro_rules! PrivatePow {
    () => {
        pub trait PrivatePow < Y , N > { type Output ; fn private_pow (self , _ : Y , _ : N) -> Self :: Output ; }
    };
}

PrivatePow!();
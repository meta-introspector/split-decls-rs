macro_rules! PrivateDivInt {
    () => {
        pub trait PrivateDivInt < C , Divisor > { type Output ; fn private_div_int (self , _ : C , _ : Divisor) -> Self :: Output ; }
    };
}

PrivateDivInt!()
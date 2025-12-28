macro_rules! PrivateDiv {
    () => {
        pub trait PrivateDiv < N , D , Q , R , I > { type Quotient ; type Remainder ; fn private_div_quotient (self , _ : N , _ : D , _ : Q , _ : R , _ : I) -> Self :: Quotient ; fn private_div_remainder (self , _ : N , _ : D , _ : Q , _ : R , _ : I) -> Self :: Remainder ; }
    };
}

PrivateDiv!();
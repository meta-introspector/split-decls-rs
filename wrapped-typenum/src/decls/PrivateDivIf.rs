macro_rules! PrivateDivIf {
    () => {
        pub trait PrivateDivIf < N , D , Q , R , I , RcmpD > { type Quotient ; type Remainder ; fn private_div_if_quotient (self , _ : N , _ : D , _ : Q , _ : R , _ : I , _ : RcmpD) -> Self :: Quotient ; fn private_div_if_remainder (self , _ : N , _ : D , _ : Q , _ : R , _ : I , _ : RcmpD) -> Self :: Remainder ; }
    };
}

PrivateDivIf!()
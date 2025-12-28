macro_rules! deps {
    () => {
        Less!();
        PrivateDivIf!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl < N , D , Q , R > PrivateDivIf < N , D , Q , R , U0 , Less > for () { type Quotient = Q ; type Remainder = R ; # [inline] fn private_div_if_quotient (self , _ : N , _ : D , q : Q , _ : R , _ : U0 , _ : Less) -> Self :: Quotient { q } # [inline] fn private_div_if_remainder (self , _ : N , _ : D , _ : Q , r : R , _ : U0 , _ : Less) -> Self :: Remainder { r } }
    };
}

impl_479!()
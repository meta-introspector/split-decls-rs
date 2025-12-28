macro_rules! deps {
    () => {
        PrivateDivIf!();
        SetBitOut!();
        Internal!();
        SetBit!();
        Greater!();
        B1!();
        Diff!();
    };
}

macro_rules! impl_481 {
    () => {
        deps!();
        impl < N , D , Q , R > PrivateDivIf < N , D , Q , R , U0 , Greater > for () where R : Sub < D > , Q : SetBit < U0 , B1 > , { type Quotient = SetBitOut < Q , U0 , B1 > ; type Remainder = Diff < R , D > ; # [inline] fn private_div_if_quotient (self , _ : N , _ : D , q : Q , _ : R , i : U0 , _ : Greater) -> Self :: Quotient { q . set_bit :: < Internal > (i , B1) } # [inline] fn private_div_if_remainder (self , _ : N , d : D , _ : Q , r : R , _ : U0 , _ : Greater ,) -> Self :: Remainder { r - d } }
    };
}

impl_481!();
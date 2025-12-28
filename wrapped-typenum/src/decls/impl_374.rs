macro_rules! deps {
    () => {
        PrivateSub!();
        UTerm!();
        Unsigned!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        # [doc = " `U - UTerm = U`"] impl < U : Unsigned > PrivateSub < UTerm > for U { type Output = U ; # [inline] fn private_sub (self , _ : UTerm) -> Self :: Output { self } }
    };
}

impl_374!();
macro_rules! deps {
    () => {
        Unsigned!();
        Bit!();
        UTerm!();
        UInt!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        # [doc = " `UInt<U, B> + UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Add < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn add (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
    };
}

impl_362!()
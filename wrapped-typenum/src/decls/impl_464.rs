macro_rules! deps {
    () => {
        UInt!();
        PrivateSetBit!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl < Un , Bn , B > PrivateSetBit < U0 , B > for UInt < Un , Bn > { type Output = UInt < Un , B > ; # [inline] fn private_set_bit (self , _ : U0 , b : B) -> Self :: Output { UInt { msb : self . msb , lsb : b , } } }
    };
}

impl_464!()
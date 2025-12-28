macro_rules! deps {
    () => {
        UInt!();
        Unsigned!();
        B1!();
        B0!();
        Add1!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        # [doc = " `UInt<U, B1> + B1 = UInt<U + B1, B0>`"] impl < U : Unsigned > Add < B1 > for UInt < U , B1 > where U : Add < B1 > , Add1 < U > : Unsigned , { type Output = UInt < Add1 < U > , B0 > ; # [inline] fn add (self , _ : B1) -> Self :: Output { UInt :: new () } }
    };
}

impl_360!()
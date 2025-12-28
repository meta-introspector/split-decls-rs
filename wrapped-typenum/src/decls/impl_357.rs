macro_rules! deps {
    () => {
        B0!();
        Bit!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        # [doc = " `U + B0 = U`"] impl < U : Unsigned , B : Bit > Add < B0 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn add (self , _ : B0) -> Self :: Output { UInt :: new () } }
    };
}

impl_357!();
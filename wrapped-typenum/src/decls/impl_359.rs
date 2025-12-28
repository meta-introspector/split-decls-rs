macro_rules! deps {
    () => {
        Unsigned!();
        B1!();
        B0!();
        UInt!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        # [doc = " `UInt<U, B0> + B1 = UInt<U + B1>`"] impl < U : Unsigned > Add < B1 > for UInt < U , B0 > { type Output = UInt < U , B1 > ; # [inline] fn add (self , _ : B1) -> Self :: Output { UInt :: new () } }
    };
}

impl_359!()
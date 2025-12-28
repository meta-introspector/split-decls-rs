macro_rules! deps {
    () => {
        Unsigned!();
        UInt!();
        B1!();
        Sub1!();
        B0!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        # [doc = " `UInt<U, B0> - B1 = UInt<U - B1, B1>`"] impl < U : Unsigned > Sub < B1 > for UInt < U , B0 > where U : Sub < B1 > , Sub1 < U > : Unsigned , { type Output = UInt < Sub1 < U > , B1 > ; # [inline] fn sub (self , _ : B1) -> Self :: Output { UInt :: new () } }
    };
}

impl_371!();
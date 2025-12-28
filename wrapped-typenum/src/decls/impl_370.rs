macro_rules! deps {
    () => {
        UInt!();
        UTerm!();
        B1!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        # [doc = " `UInt<UTerm, B1> - B1 = UTerm`"] impl Sub < B1 > for UInt < UTerm , B1 > { type Output = UTerm ; # [inline] fn sub (self , _ : B1) -> Self :: Output { UTerm } }
    };
}

impl_370!();
macro_rules! deps {
    () => {
        UInt!();
        UTerm!();
        B1!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        # [doc = " `UTerm + B1 = UInt<UTerm, B1>`"] impl Add < B1 > for UTerm { type Output = UInt < UTerm , B1 > ; # [inline] fn add (self , _ : B1) -> Self :: Output { UInt :: new () } }
    };
}

impl_358!()
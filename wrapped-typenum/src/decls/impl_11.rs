macro_rules! deps {
    () => {
        B0!();
        Bit!();
        And!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [doc = " And with 0 ( 0 & B = 0)"] impl < Rhs : Bit > BitAnd < Rhs > for B0 { type Output = B0 ; # [inline] fn bitand (self , _ : Rhs) -> Self :: Output { B0 } }
    };
}

impl_11!()
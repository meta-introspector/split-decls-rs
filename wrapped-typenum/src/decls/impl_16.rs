macro_rules! deps {
    () => {
        Bit!();
        Or!();
        B1!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [doc = " Or with 1 ( 1 | B = 1)"] impl < Rhs : Bit > BitOr < Rhs > for B1 { type Output = B1 ; # [inline] fn bitor (self , _ : Rhs) -> Self :: Output { B1 } }
    };
}

impl_16!()
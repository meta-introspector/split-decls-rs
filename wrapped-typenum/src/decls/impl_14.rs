macro_rules! deps {
    () => {
        Or!();
        B0!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Or with 0 ( 0 | 0 = 0)"] impl BitOr < B0 > for B0 { type Output = B0 ; # [inline] fn bitor (self , _ : B0) -> Self :: Output { B0 } }
    };
}

impl_14!();
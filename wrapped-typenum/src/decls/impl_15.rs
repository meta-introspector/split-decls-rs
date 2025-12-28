macro_rules! deps {
    () => {
        Or!();
        B1!();
        B0!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Or with 0 ( 0 | 1 = 1)"] impl BitOr < B1 > for B0 { type Output = B1 ; # [inline] fn bitor (self , _ : B1) -> Self :: Output { B1 } }
    };
}

impl_15!()
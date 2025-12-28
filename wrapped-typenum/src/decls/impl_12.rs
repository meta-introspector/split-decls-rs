macro_rules! deps {
    () => {
        And!();
        B1!();
        B0!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [doc = " And with 1 ( 1 & 0 = 0)"] impl BitAnd < B0 > for B1 { type Output = B0 ; # [inline] fn bitand (self , _ : B0) -> Self :: Output { B0 } }
    };
}

impl_12!();
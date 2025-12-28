macro_rules! deps {
    () => {
        And!();
        B0!();
        B1!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [doc = " And with 1 ( 1 & 0 = 0)"] impl BitAnd < B0 > for B1 { type Output = B0 ; # [inline] fn bitand (self , _ : B0) -> Self :: Output { B0 } }
    };
}

impl_12!()
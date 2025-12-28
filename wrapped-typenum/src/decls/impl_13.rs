macro_rules! deps {
    () => {
        And!();
        B1!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [doc = " And with 1 ( 1 & 1 = 1)"] impl BitAnd < B1 > for B1 { type Output = B1 ; # [inline] fn bitand (self , _ : B1) -> Self :: Output { B1 } }
    };
}

impl_13!()
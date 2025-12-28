macro_rules! deps {
    () => {
        Xor!();
        B0!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [doc = " Xor between 0 and 0 ( 0 ^ 0 = 0)"] impl BitXor < B0 > for B0 { type Output = B0 ; # [inline] fn bitxor (self , _ : B0) -> Self :: Output { B0 } }
    };
}

impl_17!()
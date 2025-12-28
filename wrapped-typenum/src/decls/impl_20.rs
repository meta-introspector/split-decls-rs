macro_rules! deps {
    () => {
        Xor!();
        B0!();
        B1!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [doc = " Xor between 1 and 1 ( 1 ^ 1 = 0)"] impl BitXor < B1 > for B1 { type Output = B0 ; # [inline] fn bitxor (self , _ : B1) -> Self :: Output { B0 } }
    };
}

impl_20!()
macro_rules! deps {
    () => {
        B0!();
        Xor!();
        B1!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [doc = " Xor between 1 and 0 ( 1 ^ 0 = 1)"] impl BitXor < B0 > for B1 { type Output = B1 ; # [inline] fn bitxor (self , _ : B0) -> Self :: Output { B1 } }
    };
}

impl_18!();
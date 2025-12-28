macro_rules! deps {
    () => {
        Xor!();
        B0!();
        B1!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [doc = " Xor between 0 and 1 ( 0 ^ 1 = 1)"] impl BitXor < B1 > for B0 { type Output = B1 ; # [inline] fn bitxor (self , _ : B1) -> Self :: Output { B1 } }
    };
}

impl_19!()
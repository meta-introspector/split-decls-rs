macro_rules! deps {
    () => {
        B1!();
        UTerm!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        # [doc = " Shifting `UTerm` by a 1 bit: `UTerm << B1 = UTerm`"] impl Shl < B1 > for UTerm { type Output = UTerm ; # [inline] fn shl (self , _ : B1) -> Self :: Output { UTerm } }
    };
}

impl_402!();
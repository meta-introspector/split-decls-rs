macro_rules! deps {
    () => {
        B0!();
        UTerm!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        # [doc = " Shifting `UTerm` by a 0 bit: `UTerm << B0 = UTerm`"] impl Shl < B0 > for UTerm { type Output = UTerm ; # [inline] fn shl (self , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_401!();
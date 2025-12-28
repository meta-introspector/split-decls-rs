macro_rules! deps {
    () => {
        UTerm!();
        B1!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        # [doc = " Shifting right `UTerm` by a 1 bit: `UTerm >> B1 = UTerm`"] impl Shr < B1 > for UTerm { type Output = UTerm ; # [inline] fn shr (self , _ : B1) -> Self :: Output { UTerm } }
    };
}

impl_411!()
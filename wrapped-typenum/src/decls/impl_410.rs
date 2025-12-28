macro_rules! deps {
    () => {
        UTerm!();
        B0!();
    };
}

macro_rules! impl_410 {
    () => {
        deps!();
        # [doc = " Shifting right `UTerm` by a 0 bit: `UTerm >> B0 = UTerm`"] impl Shr < B0 > for UTerm { type Output = UTerm ; # [inline] fn shr (self , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_410!();
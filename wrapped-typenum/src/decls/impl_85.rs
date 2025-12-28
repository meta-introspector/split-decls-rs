macro_rules! deps {
    () => {
        Integer!();
        NonZero!();
        Z0!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        # [doc = " `Z0 / I = Z0` where `I != 0`"] impl < I : Integer + NonZero > Div < I > for Z0 { type Output = Z0 ; # [inline] fn div (self , _ : I) -> Self :: Output { Z0 } }
    };
}

impl_85!()
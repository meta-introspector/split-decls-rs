macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        # [doc = " Shifting right a `UTerm` by an unsigned integer: `UTerm >> U = UTerm`"] impl < U : Unsigned > Shr < U > for UTerm { type Output = UTerm ; # [inline] fn shr (self , _ : U) -> Self :: Output { UTerm } }
    };
}

impl_408!();
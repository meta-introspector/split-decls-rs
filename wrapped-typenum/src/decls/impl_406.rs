macro_rules! deps {
    () => {
        UTerm!();
        Unsigned!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        # [doc = " Shifting left `UTerm` by an unsigned integer: `UTerm << U = UTerm`"] impl < U : Unsigned > Shl < U > for UTerm { type Output = UTerm ; # [inline] fn shl (self , _ : U) -> Self :: Output { UTerm } }
    };
}

impl_406!()
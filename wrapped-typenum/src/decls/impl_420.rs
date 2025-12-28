macro_rules! deps {
    () => {
        UTerm!();
        Unsigned!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        # [doc = " `UTerm * U = UTerm`"] impl < U : Unsigned > Mul < U > for UTerm { type Output = UTerm ; # [inline] fn mul (self , _ : U) -> Self :: Output { UTerm } }
    };
}

impl_420!()
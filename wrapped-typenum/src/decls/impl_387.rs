macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        # [doc = " `UTerm | X = X`"] impl < U : Unsigned > BitOr < U > for UTerm { type Output = U ; # [inline] fn bitor (self , rhs : U) -> Self :: Output { rhs } }
    };
}

impl_387!();
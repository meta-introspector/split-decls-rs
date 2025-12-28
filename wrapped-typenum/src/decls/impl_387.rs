macro_rules! deps {
    () => {
        UTerm!();
        Unsigned!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        # [doc = " `UTerm | X = X`"] impl < U : Unsigned > BitOr < U > for UTerm { type Output = U ; # [inline] fn bitor (self , rhs : U) -> Self :: Output { rhs } }
    };
}

impl_387!()
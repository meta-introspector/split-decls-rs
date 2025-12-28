macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        # [doc = " `UTerm + U = U`"] impl < U : Unsigned > Add < U > for UTerm { type Output = U ; # [inline] fn add (self , rhs : U) -> Self :: Output { rhs } }
    };
}

impl_361!();
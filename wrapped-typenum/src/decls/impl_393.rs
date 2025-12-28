macro_rules! deps {
    () => {
        UTerm!();
        Unsigned!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        # [doc = " 0 ^ X = X"] impl < Ur : Unsigned > BitXor < Ur > for UTerm { type Output = Ur ; # [inline] fn bitxor (self , rhs : Ur) -> Self :: Output { rhs } }
    };
}

impl_393!();
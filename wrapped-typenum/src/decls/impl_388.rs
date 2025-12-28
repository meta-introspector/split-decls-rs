macro_rules! deps {
    () => {
        UTerm!();
        UInt!();
        Bit!();
        Unsigned!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        # [doc = "  `X | UTerm = X`"] impl < B : Bit , U : Unsigned > BitOr < UTerm > for UInt < U , B > { type Output = Self ; # [inline] fn bitor (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
    };
}

impl_388!();
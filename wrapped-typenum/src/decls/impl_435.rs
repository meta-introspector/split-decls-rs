macro_rules! deps {
    () => {
        Ord!();
        Bit!();
        UTerm!();
        UInt!();
        Unsigned!();
        PrivateCmp!();
        Greater!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        # [doc = " Got to the end of just the `Rhs`. `Lhs` is `Greater`."] impl < U : Unsigned , B : Bit , SoFar : Ord > PrivateCmp < UTerm , SoFar > for UInt < U , B > { type Output = Greater ; # [inline] fn private_cmp (& self , _ : & UTerm , _ : SoFar) -> Self :: Output { Greater } }
    };
}

impl_435!()
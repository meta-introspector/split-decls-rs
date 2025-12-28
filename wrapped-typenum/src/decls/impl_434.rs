macro_rules! deps {
    () => {
        UInt!();
        Bit!();
        Ord!();
        Unsigned!();
        PrivateCmp!();
        Less!();
        UTerm!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        # [doc = " Got to the end of just the `Lhs`. It's `Less`."] impl < U : Unsigned , B : Bit , SoFar : Ord > PrivateCmp < UInt < U , B > , SoFar > for UTerm { type Output = Less ; # [inline] fn private_cmp (& self , _ : & UInt < U , B > , _ : SoFar) -> Self :: Output { Less } }
    };
}

impl_434!()
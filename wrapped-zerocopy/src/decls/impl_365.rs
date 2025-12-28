macro_rules! deps {
    () => {
        InvariantsEq!();
        TransmuteFrom!();
        BecauseInvariantsEq!();
        Validity!();
        Aliasing!();
        MutationCompatible!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        unsafe impl < Src : ? Sized , Dst : ? Sized , A : Aliasing , SV : Validity , DV : Validity > MutationCompatible < Src , A , SV , DV , BecauseInvariantsEq > for Dst where Src : TransmuteFrom < Dst , DV , SV > , Dst : TransmuteFrom < Src , SV , DV > + InvariantsEq < Src > , { }
    };
}

impl_365!()
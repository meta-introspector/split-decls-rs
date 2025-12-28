macro_rules! deps {
    () => {
        SizeEq!();
        TransmuteFrom!();
        MutationCompatible!();
        TryTransmuteFromPtr!();
        BecauseMutationCompatible!();
        Aliasing!();
        Validity!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        unsafe impl < Src , Dst , SV , DV , A , R > TryTransmuteFromPtr < Src , A , SV , DV , (BecauseMutationCompatible , R) > for Dst where A : Aliasing , SV : Validity , DV : Validity , Src : TransmuteFrom < Dst , DV , SV > + ? Sized , Dst : MutationCompatible < Src , A , SV , DV , R > + SizeEq < Src > + ? Sized , { }
    };
}

impl_358!();
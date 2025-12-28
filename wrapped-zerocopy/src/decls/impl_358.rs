macro_rules! deps {
    () => {
        Aliasing!();
        Validity!();
        TryTransmuteFromPtr!();
        SizeEq!();
        BecauseMutationCompatible!();
        MutationCompatible!();
        TransmuteFrom!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        unsafe impl < Src , Dst , SV , DV , A , R > TryTransmuteFromPtr < Src , A , SV , DV , (BecauseMutationCompatible , R) > for Dst where A : Aliasing , SV : Validity , DV : Validity , Src : TransmuteFrom < Dst , DV , SV > + ? Sized , Dst : MutationCompatible < Src , A , SV , DV , R > + SizeEq < Src > + ? Sized , { }
    };
}

impl_358!()
macro_rules! deps {
    () => {
        Validity!();
        TryTransmuteFromPtr!();
        TransmuteFrom!();
        Aliasing!();
        TransmuteFromPtr!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        unsafe impl < Src : ? Sized , Dst : ? Sized , A : Aliasing , SV : Validity , DV : Validity , R > TransmuteFromPtr < Src , A , SV , DV , R > for Dst where Dst : TransmuteFrom < Src , SV , DV > + TryTransmuteFromPtr < Src , A , SV , DV , R > , { }
    };
}

impl_373!()
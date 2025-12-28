macro_rules! deps {
    () => {
        Validity!();
        Read!();
        Aliasing!();
        BecauseRead!();
        MutationCompatible!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        unsafe impl < Src : ? Sized , Dst : ? Sized , A : Aliasing , SV : Validity , DV : Validity , R , S > MutationCompatible < Src , A , SV , DV , (BecauseRead , (R , S)) > for Dst where Src : Read < A , R > , Dst : Read < A , S > , { }
    };
}

impl_362!();
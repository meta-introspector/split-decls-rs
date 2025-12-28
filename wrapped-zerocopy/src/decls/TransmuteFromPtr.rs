macro_rules! deps {
    () => {
        TransmuteFrom!();
        Aliasing!();
        Validity!();
        TryTransmuteFromPtr!();
    };
}

macro_rules! TransmuteFromPtr {
    () => {
        deps!();
        # [doc = " Transmutations which are always sound."] # [doc = ""] # [doc = " `TransmuteFromPtr` is a shorthand for [`TryTransmuteFromPtr`] and"] # [doc = " [`TransmuteFrom`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `Dst: TransmuteFromPtr<Src, A, SV, DV, _>` is equivalent to `Dst:"] # [doc = " TryTransmuteFromPtr<Src, A, SV, DV, _> + TransmuteFrom<Src, SV, DV>`."] pub unsafe trait TransmuteFromPtr < Src : ? Sized , A : Aliasing , SV : Validity , DV : Validity , R > : TryTransmuteFromPtr < Src , A , SV , DV , R > + TransmuteFrom < Src , SV , DV > { }
    };
}

TransmuteFromPtr!();
macro_rules! TransmuteFrom {
    () => {
        # [doc = " Denotes that any `SV`-valid `Src` may soundly be transmuted into a"] # [doc = " `DV`-valid `Self`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Given `src: Ptr<Src, (_, _, SV)>` and `dst: Ptr<Dst, (_, _, DV)>`, if the"] # [doc = " referents of `src` and `dst` are the same size, then the set of bit patterns"] # [doc = " allowed to appear in `src`'s referent must be a subset of the set allowed to"] # [doc = " appear in `dst`'s referent."] # [doc = ""] # [doc = " If the referents are not the same size, then `Dst: TransmuteFrom<Src, SV,"] # [doc = " DV>` conveys no safety guarantee."] pub unsafe trait TransmuteFrom < Src : ? Sized , SV , DV > { }
    };
}

TransmuteFrom!();
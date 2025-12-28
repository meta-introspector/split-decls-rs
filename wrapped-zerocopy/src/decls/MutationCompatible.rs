macro_rules! deps {
    () => {
        Aliasing!();
    };
}

macro_rules! MutationCompatible {
    () => {
        deps!();
        # [doc = " Denotes that `src: Ptr<Src, (A, _, SV)>` and `dst: Ptr<Self, (A, _, DV)>`,"] # [doc = " referencing the same referent at the same time, cannot be used by safe code"] # [doc = " to break library safety invariants of `Src` or `Self`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " At least one of the following must hold:"] # [doc = " - `Src: Read<A, _>` and `Self: Read<A, _>`"] # [doc = " - `Self: InvariantsEq<Src>`, and, for some `V`:"] # [doc = "   - `Dst: TransmuteFrom<Src, V, V>`"] # [doc = "   - `Src: TransmuteFrom<Dst, V, V>`"] pub unsafe trait MutationCompatible < Src : ? Sized , A : Aliasing , SV , DV , R > { }
    };
}

MutationCompatible!()
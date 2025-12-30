// Generated macro for TransmuteFromPtr (trait)
macro_rules! Depcrate_pointer_transmuteTransmuteFromPtr {
() => {
// Module: crate::pointer::transmute
// Provides: {"TransmuteFromPtr"}
// Dependencies: {}
# [doc = " Transmutations which are always sound."] # [doc = ""] # [doc = " `TransmuteFromPtr` is a shorthand for [`TryTransmuteFromPtr`] and"] # [doc = " [`TransmuteFrom`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `Dst: TransmuteFromPtr<Src, A, SV, DV, _>` is equivalent to `Dst:"] # [doc = " TryTransmuteFromPtr<Src, A, SV, DV, _> + TransmuteFrom<Src, SV, DV>`."] pub unsafe trait TransmuteFromPtr < Src : ? Sized , A : Aliasing , SV : Validity , DV : Validity , R > : TryTransmuteFromPtr < Src , A , SV , DV , R > + TransmuteFrom < Src , SV , DV > { }
};
}

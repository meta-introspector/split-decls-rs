// Generated macro for CastableFrom (trait)
macro_rules! Depcrate_pointer_invariantCastableFrom {
() => {
// Module: crate::pointer::invariant
// Provides: {"CastableFrom"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " `DT: CastableFrom<ST, SV, DV>` is sound if `SV = DV = Uninit` or `SV = DV ="] # [doc = " Initialized`."] pub unsafe trait CastableFrom < ST : ? Sized , SV , DV > { }
};
}

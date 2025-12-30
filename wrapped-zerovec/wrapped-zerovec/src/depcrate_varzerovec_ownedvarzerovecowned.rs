// Generated macro for VarZeroVecOwned (struct)
macro_rules! Depcrate_varzerovec_ownedVarZeroVecOwned {
() => {
// Module: crate::varzerovec::owned
// Provides: {"VarZeroVecOwned"}
// Dependencies: {}
# [doc = " A fully-owned [`VarZeroVec`]. This type has no lifetime but has the same"] # [doc = " internal buffer representation of [`VarZeroVec`], making it cheaply convertible to"] # [doc = " [`VarZeroVec`] and [`VarZeroSlice`]."] # [doc = ""] # [doc = " The `F` type parameter is a [`VarZeroVecFormat`] (see its docs for more details), which can be used to select the"] # [doc = " precise format of the backing buffer with various size and performance tradeoffs. It defaults to [`Index16`]."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub struct VarZeroVecOwned < T : ? Sized , F = Index16 > { marker1 : PhantomData < T > , marker2 : PhantomData < F > , entire_slice : Vec < u8 > , }
};
}

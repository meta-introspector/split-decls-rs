// Generated macro for vecs (module)
macro_rules! Depcratevecs {
() => {
// Module: crate
// Provides: {"vecs"}
// Dependencies: {}
pub mod vecs { # ! [doc = " This module contains additional utility types for working with"] # ! [doc = " [`ZeroVec`] and  [`VarZeroVec`]. See their docs for more details on the general purpose"] # ! [doc = " of these types."] # ! [doc = ""] # ! [doc = " [`ZeroSlice`] and [`VarZeroSlice`] provide slice-like versions of the vector types"] # ! [doc = " for use behind references and in custom ULE types."] # ! [doc = ""] # ! [doc = " [`VarZeroVecOwned`] is a special owned/mutable version of [`VarZeroVec`], allowing"] # ! [doc = " direct manipulation of the backing buffer."] # [doc (no_inline)] pub use crate :: zerovec :: { ZeroSlice , ZeroVec } ; pub use crate :: zerovec :: ZeroSliceIter ; # [doc (no_inline)] pub use crate :: varzerovec :: { VarZeroSlice , VarZeroVec } ; # [cfg (feature = "alloc")] pub use crate :: varzerovec :: VarZeroVecOwned ; pub use crate :: varzerovec :: { Index16 , Index32 , Index8 , VarZeroSliceIter , VarZeroVecFormat } ; pub type VarZeroVec16 < 'a , T > = VarZeroVec < 'a , T , Index16 > ; pub type VarZeroVec32 < 'a , T > = VarZeroVec < 'a , T , Index32 > ; pub type VarZeroSlice16 < T > = VarZeroSlice < T , Index16 > ; pub type VarZeroSlice32 < T > = VarZeroSlice < T , Index32 > ; }
};
}

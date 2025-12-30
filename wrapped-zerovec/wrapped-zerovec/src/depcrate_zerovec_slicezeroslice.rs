// Generated macro for ZeroSlice (struct)
macro_rules! Depcrate_zerovec_sliceZeroSlice {
() => {
// Module: crate::zerovec::slice
// Provides: {"ZeroSlice"}
// Dependencies: {}
# [doc = " A zero-copy \"slice\", i.e. the zero-copy version of `[T]`."] # [doc = ""] # [doc = " This behaves"] # [doc = " similarly to [`ZeroVec<T>`], however [`ZeroVec<T>`] is allowed to contain"] # [doc = " owned data and as such is ideal for deserialization since most human readable"] # [doc = " serialization formats cannot unconditionally deserialize zero-copy."] # [doc = ""] # [doc = " This type can be used inside [`VarZeroVec<T>`](crate::VarZeroVec) and [`ZeroMap`](crate::ZeroMap):"] # [doc = " This essentially allows for the construction of zero-copy types isomorphic to `Vec<Vec<T>>` by instead"] # [doc = " using `VarZeroVec<ZeroSlice<T>>`. See the [`VarZeroVec`](crate::VarZeroVec) docs for an example."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Const-construct a ZeroSlice of u16:"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::ule::AsULE;"] # [doc = " use zerovec::ZeroSlice;"] # [doc = ""] # [doc = " const DATA: &ZeroSlice<u16> ="] # [doc = "     ZeroSlice::<u16>::from_ule_slice(&<u16 as AsULE>::ULE::from_array(["] # [doc = "         211, 281, 421, 32973,"] # [doc = "     ]));"] # [doc = ""] # [doc = " assert_eq!(DATA.get(1), Some(281));"] # [doc = " ```"] # [repr (transparent)] pub struct ZeroSlice < T : AsULE > ([T :: ULE]) ;
};
}

// Generated macro for impl_580 (impl)
macro_rules! Depcrate_arrayimpl_580 {
() => {
// Module: crate::array
// Provides: {"impl_580"}
// Dependencies: {}
# [doc = " Size of a `TypeArray`"] impl < V , A > Len for TArr < V , A > where A : Len , Length < A > : Add < B1 > , Sum < Length < A > , B1 > : Unsigned , { type Output = Add1 < Length < A > > ; # [inline] fn len (& self) -> Self :: Output { self . rest . len () + B1 } }
};
}

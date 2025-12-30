// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < T : ConstantTimeEq > ConstantTimeEq for [T] { # [doc = " Check whether two slices of `ConstantTimeEq` types are equal."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function short-circuits if the lengths of the input slices"] # [doc = " are different.  Otherwise, it should execute in time independent"] # [doc = " of the slice contents."] # [doc = ""] # [doc = " Since arrays coerce to slices, this function works with fixed-size arrays:"] # [doc = ""] # [doc = " ```"] # [doc = " # use subtle::ConstantTimeEq;"] # [doc = " #"] # [doc = " let a: [u8; 8] = [0,1,2,3,4,5,6,7];"] # [doc = " let b: [u8; 8] = [0,1,2,3,0,1,2,3];"] # [doc = ""] # [doc = " let a_eq_a = a.ct_eq(&a);"] # [doc = " let a_eq_b = a.ct_eq(&b);"] # [doc = ""] # [doc = " assert_eq!(a_eq_a.unwrap_u8(), 1);"] # [doc = " assert_eq!(a_eq_b.unwrap_u8(), 0);"] # [doc = " ```"] # [inline] fn ct_eq (& self , _rhs : & [T]) -> Choice { let len = self . len () ; if len != _rhs . len () { return Choice :: from (0) ; } let mut x = 1u8 ; for (ai , bi) in self . iter () . zip (_rhs . iter ()) { x &= ai . ct_eq (bi) . unwrap_u8 () ; } x . into () } }
};
}

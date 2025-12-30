// Generated macro for impl_29 (impl)
macro_rules! Depcrate_arrayvecimpl_29 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_29"}
// Dependencies: {}
impl < A > ArrayVec < A > { # [doc = " Wraps up an array as a new empty `ArrayVec`."] # [doc = ""] # [doc = " If you want to simply use the full array, use `from` instead."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " This method in particular allows to create values for statics:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use tinyvec::ArrayVec;"] # [doc = " static DATA: ArrayVec<[u8; 5]> = ArrayVec::from_array_empty([0; 5]);"] # [doc = " assert_eq!(DATA.len(), 0);"] # [doc = " ```"] # [doc = ""] # [doc = " But of course it is just an normal empty `ArrayVec`:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use tinyvec::ArrayVec;"] # [doc = " let mut data = ArrayVec::from_array_empty([1, 2, 3, 4]);"] # [doc = " assert_eq!(&data[..], &[]);"] # [doc = " data.push(42);"] # [doc = " assert_eq!(&data[..], &[42]);"] # [doc = " ```"] # [inline] # [must_use] pub const fn from_array_empty (data : A) -> Self { Self { data , len : 0 } } }
};
}

// Generated macro for impl_51 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_51 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , const D : usize > MatrixBorrowed < 'a , D > { # [cfg (debug_assertions)] pub (super) fn debug_assert_dims (& self , dims : [usize ; D]) { debug_assert_eq ! (dims , self . dims) ; let expected_len = dims . iter () . product :: < usize > () ; debug_assert_eq ! (expected_len , self . data . len ()) ; } pub (super) fn as_slice (& self) -> & 'a [f32] { self . data } # [doc = " See [`MatrixOwned::submatrix`]."] # [inline] pub (super) fn submatrix < const M : usize > (& self , index : usize) -> Option < MatrixBorrowed < 'a , M > > { assert_eq ! (M , D - 1) ; let (range , dims) = self . submatrix_range (index) ; let data = & self . data . get (range) ? ; Some (MatrixBorrowed { data , dims }) } # [inline] fn submatrix_range < const M : usize > (& self , index : usize) -> (Range < usize > , [usize ; M]) { assert_eq ! (M , D - 1) ; # [expect (clippy :: unwrap_used)] let sub_dims : [usize ; M] = self . dims [1 ..] . try_into () . unwrap () ; let n = sub_dims . iter () . product :: < usize > () ; (n * index .. n * (index + 1) , sub_dims) } }
};
}

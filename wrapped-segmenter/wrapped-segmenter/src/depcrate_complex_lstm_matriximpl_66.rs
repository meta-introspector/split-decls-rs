// Generated macro for impl_66 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_66 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a , const D : usize > MatrixZero < 'a , D > { # [expect (clippy :: wrong_self_convention)] pub (super) fn to_owned (& self) -> MatrixOwned < D > { MatrixOwned { data : self . data . iter () . collect () , dims : self . dims , } } pub (super) fn as_slice (& self) -> & ZeroSlice < f32 > { self . data } # [cfg (debug_assertions)] pub (super) fn debug_assert_dims (& self , dims : [usize ; D]) { debug_assert_eq ! (dims , self . dims) ; let expected_len = dims . iter () . product :: < usize > () ; debug_assert_eq ! (expected_len , self . data . len ()) ; } # [doc = " See [`MatrixOwned::submatrix`]."] # [inline] pub (super) fn submatrix < const M : usize > (& self , index : usize) -> Option < MatrixZero < 'a , M > > { assert_eq ! (M , D - 1) ; let (range , dims) = self . submatrix_range (index) ; let data = & self . data . get_subslice (range) ? ; Some (MatrixZero { data , dims }) } # [inline] fn submatrix_range < const M : usize > (& self , index : usize) -> (Range < usize > , [usize ; M]) { assert_eq ! (M , D - 1) ; # [expect (clippy :: unwrap_used)] let sub_dims : [usize ; M] = self . dims [1 ..] . try_into () . unwrap () ; let n = sub_dims . iter () . product :: < usize > () ; (n * index .. n * (index + 1) , sub_dims) } }
};
}

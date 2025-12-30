// Generated macro for impl_60 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_60 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_60"}
// Dependencies: {}
impl MatrixBorrowedMut < '_ , 1 > { # [doc = " Calculate the dot product of a and b, adding the result to self."] # [doc = ""] # [doc = " Note: For better dot product efficiency, if `b` is MxN, then `a` should be N;"] # [doc = " this is the opposite of standard practice."] pub (super) fn add_dot_2d (& mut self , a : MatrixBorrowed < 1 > , b : MatrixZero < 2 >) { let m = a . dim () ; let n = self . as_borrowed () . dim () ; debug_assert_eq ! (m , b . dim () . 1 , "dims: {:?}/{:?}/{:?}" , self . as_borrowed () . dim () , a . dim () , b . dim ()) ; debug_assert_eq ! (n , b . dim () . 0 , "dims: {:?}/{:?}/{:?}" , self . as_borrowed () . dim () , a . dim () , b . dim ()) ; for i in 0 .. n { if let (Some (dest) , Some (b_sub)) = (self . as_mut_slice () . get_mut (i) , b . submatrix :: < 1 > (i)) { * dest += unrolled_dot_1 (a . data , b_sub . data) ; } else { debug_assert ! (false , "unreachable: dims checked above") ; } } } }
};
}

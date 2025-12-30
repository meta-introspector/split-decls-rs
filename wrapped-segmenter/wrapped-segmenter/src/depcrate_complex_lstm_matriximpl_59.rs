// Generated macro for impl_59 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_59 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_59"}
// Dependencies: {}
impl MatrixBorrowed < '_ , 1 > { # [allow (dead_code)] pub (super) fn dot_1d (& self , other : MatrixZero < 1 >) -> f32 { debug_assert_eq ! (self . dims , other . dims) ; unrolled_dot_1 (self . data , other . data) } }
};
}

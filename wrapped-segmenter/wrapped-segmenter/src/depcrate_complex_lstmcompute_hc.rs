// Generated macro for compute_hc (function)
macro_rules! Depcrate_complex_lstmcompute_hc {
() => {
// Module: crate::complex::lstm
// Provides: {"compute_hc"}
// Dependencies: {}
# [doc = " `compute_hc1` implemens the evaluation of one LSTM layer."] fn compute_hc < 'a > (x_t : MatrixZero < 'a , 1 > , mut h_tm1 : MatrixBorrowedMut < 'a , 1 > , mut c_tm1 : MatrixBorrowedMut < 'a , 1 > , w : MatrixZero < 'a , 3 > , u : MatrixZero < 'a , 3 > , b : MatrixZero < 'a , 2 > ,) { # [cfg (debug_assertions)] { let hunits = h_tm1 . dim () ; let embedd_dim = x_t . dim () ; c_tm1 . as_borrowed () . debug_assert_dims ([hunits]) ; w . debug_assert_dims ([4 , hunits , embedd_dim]) ; u . debug_assert_dims ([4 , hunits , hunits]) ; b . debug_assert_dims ([4 , hunits]) ; } let mut s_t = b . to_owned () ; s_t . as_mut () . add_dot_3d_2 (x_t , w) ; s_t . as_mut () . add_dot_3d_1 (h_tm1 . as_borrowed () , u) ; # [expect (clippy :: unwrap_used)] s_t . submatrix_mut :: < 1 > (0) . unwrap () . sigmoid_transform () ; # [expect (clippy :: unwrap_used)] s_t . submatrix_mut :: < 1 > (1) . unwrap () . sigmoid_transform () ; # [expect (clippy :: unwrap_used)] s_t . submatrix_mut :: < 1 > (2) . unwrap () . tanh_transform () ; # [expect (clippy :: unwrap_used)] s_t . submatrix_mut :: < 1 > (3) . unwrap () . sigmoid_transform () ; # [expect (clippy :: unwrap_used)] c_tm1 . convolve (s_t . as_borrowed () . submatrix (0) . unwrap () , s_t . as_borrowed () . submatrix (2) . unwrap () , s_t . as_borrowed () . submatrix (1) . unwrap () ,) ; # [expect (clippy :: unwrap_used)] h_tm1 . mul_tanh (s_t . as_borrowed () . submatrix (3) . unwrap () , c_tm1 . as_borrowed ()) ; }
};
}

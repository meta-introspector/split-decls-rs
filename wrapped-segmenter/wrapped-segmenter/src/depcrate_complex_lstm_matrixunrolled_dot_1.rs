// Generated macro for unrolled_dot_1 (function)
macro_rules! Depcrate_complex_lstm_matrixunrolled_dot_1 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"unrolled_dot_1"}
// Dependencies: {}
# [doc = " Compute the dot product of an aligned and an unaligned f32 slice."] # [doc = ""] # [doc = " `xs` and `ys` must be the same length"] # [doc = ""] # [doc = " (Based on ndarray 0.15.6)"] fn unrolled_dot_1 (xs : & [f32] , ys : & ZeroSlice < f32 >) -> f32 { debug_assert_eq ! (xs . len () , ys . len ()) ; let mut p = (0.0 , 0.0 , 0.0 , 0.0 , 0.0 , 0.0 , 0.0 , 0.0) ; let xit = xs . chunks_exact (8) ; let yit = ys . as_ule_slice () . chunks_exact (8) ; let sum = xit . remainder () . iter () . zip (yit . remainder () . iter ()) . map (| (x , y) | x * f32c ! (* y)) . sum :: < f32 > () ; for (xx , yy) in xit . zip (yit) { # [expect (clippy :: unwrap_used)] let [x0 , x1 , x2 , x3 , x4 , x5 , x6 , x7] = * < & [f32 ; 8] > :: try_from (xx) . unwrap () ; # [expect (clippy :: unwrap_used)] let [y0 , y1 , y2 , y3 , y4 , y5 , y6 , y7] = * < & [< f32 as AsULE > :: ULE ; 8] > :: try_from (yy) . unwrap () ; p . 0 += x0 * f32c ! (y0) ; p . 1 += x1 * f32c ! (y1) ; p . 2 += x2 * f32c ! (y2) ; p . 3 += x3 * f32c ! (y3) ; p . 4 += x4 * f32c ! (y4) ; p . 5 += x5 * f32c ! (y5) ; p . 6 += x6 * f32c ! (y6) ; p . 7 += x7 * f32c ! (y7) ; } sum + (p . 0 + p . 4) + (p . 1 + p . 5) + (p . 2 + p . 6) + (p . 3 + p . 7) }
};
}

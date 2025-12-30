// Generated macro for MatrixZero (struct)
macro_rules! Depcrate_complex_lstm_matrixMatrixZero {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"MatrixZero"}
// Dependencies: {}
# [doc = " A `D`-dimensional matrix borrowed from a [`ZeroSlice`]."] # [derive (Debug , Clone , Copy)] pub (super) struct MatrixZero < 'a , const D : usize > { data : & 'a ZeroSlice < f32 > , dims : [usize ; D] , }
};
}

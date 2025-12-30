// Generated macro for MatrixOwned (struct)
macro_rules! Depcrate_complex_lstm_matrixMatrixOwned {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"MatrixOwned"}
// Dependencies: {}
# [doc = " A `D`-dimensional, heap-allocated matrix."] # [doc = ""] # [doc = " This matrix implementation supports slicing matrices into tightly-packed"] # [doc = " submatrices. For example, indexing into a matrix of size 5x4x3 returns a"] # [doc = " matrix of size 4x3. For more information, see [`MatrixOwned::submatrix`]."] # [derive (Debug , Clone)] pub (super) struct MatrixOwned < const D : usize > { data : Vec < f32 > , dims : [usize ; D] , }
};
}

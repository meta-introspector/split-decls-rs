// Generated macro for impl_65 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_65 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a > From < & 'a crate :: provider :: LstmMatrix3 < 'a > > for MatrixZero < 'a , 3 > { fn from (other : & 'a crate :: provider :: LstmMatrix3 < 'a >) -> Self { Self { data : & other . data , dims : other . dims . map (| x | x as usize) , } } }
};
}

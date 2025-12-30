// Generated macro for impl_63 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_63 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > From < & 'a crate :: provider :: LstmMatrix1 < 'a > > for MatrixZero < 'a , 1 > { fn from (other : & 'a crate :: provider :: LstmMatrix1 < 'a >) -> Self { Self { data : & other . data , dims : other . dims . map (| x | x as usize) , } } }
};
}

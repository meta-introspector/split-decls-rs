// Generated macro for impl_64 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_64 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > From < & 'a crate :: provider :: LstmMatrix2 < 'a > > for MatrixZero < 'a , 2 > { fn from (other : & 'a crate :: provider :: LstmMatrix2 < 'a >) -> Self { Self { data : & other . data , dims : other . dims . map (| x | x as usize) , } } }
};
}

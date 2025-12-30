// Generated macro for impl_78 (impl)
macro_rules! Depcrate_complex_lstmimpl_78 {
() => {
// Module: crate::complex::lstm
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'l , 'data > BiesIterator < 'l , 'data > { fn new (segmenter : & 'l LstmSegmenter < 'data > , input_seq : Vec < u16 >) -> Self { let hunits = segmenter . fw_u . dim () . 1 ; let mut c_bw = MatrixOwned :: < 1 > :: new_zero ([hunits]) ; let mut h_bw = MatrixOwned :: < 2 > :: new_zero ([input_seq . len () , hunits]) ; for (i , & g_id) in input_seq . iter () . enumerate () . rev () { if i + 1 < input_seq . len () { h_bw . as_mut () . copy_submatrix :: < 1 > (i + 1 , i) ; } # [expect (clippy :: unwrap_used)] compute_hc (segmenter . embedding . submatrix :: < 1 > (g_id as usize) . unwrap () , h_bw . submatrix_mut (i) . unwrap () , c_bw . as_mut () , segmenter . bw_w , segmenter . bw_u , segmenter . bw_b ,) ; } Self { input_seq : input_seq . into_iter () . enumerate () , h_bw , c_fw : MatrixOwned :: < 1 > :: new_zero ([hunits]) , curr_fw : MatrixOwned :: < 1 > :: new_zero ([hunits]) , segmenter , } } }
};
}

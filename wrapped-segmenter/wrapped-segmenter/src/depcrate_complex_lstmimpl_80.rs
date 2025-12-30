// Generated macro for impl_80 (impl)
macro_rules! Depcrate_complex_lstmimpl_80 {
() => {
// Module: crate::complex::lstm
// Provides: {"impl_80"}
// Dependencies: {}
impl Iterator for BiesIterator < '_ , '_ > { type Item = bool ; fn next (& mut self) -> Option < Self :: Item > { let (i , g_id) = self . input_seq . next () ? ; # [expect (clippy :: unwrap_used)] compute_hc (self . segmenter . embedding . submatrix :: < 1 > (g_id as usize) . unwrap () , self . curr_fw . as_mut () , self . c_fw . as_mut () , self . segmenter . fw_w , self . segmenter . fw_u , self . segmenter . fw_b ,) ; # [expect (clippy :: unwrap_used)] let curr_bw = self . h_bw . submatrix :: < 1 > (i) . unwrap () ; let mut weights = [0.0 ; 4] ; let mut curr_est = MatrixBorrowedMut { data : & mut weights , dims : [4] , } ; curr_est . add_dot_2d (self . curr_fw . as_borrowed () , self . segmenter . timew_fw) ; curr_est . add_dot_2d (curr_bw , self . segmenter . timew_bw) ; # [expect (clippy :: unwrap_used)] curr_est . add (self . segmenter . time_b) . unwrap () ; Some (weights [2] > weights [0] && weights [2] > weights [1] && weights [2] > weights [3]) } }
};
}

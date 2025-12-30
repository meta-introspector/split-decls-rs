// Generated macro for impl_107 (impl)
macro_rules! Depcrate_direntimpl_107 {
() => {
// Module: crate::dirent
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > Buffer < 'a > for & 'a mut [u8] { fn empty () -> Self { & mut [] } fn length (& self) -> usize { self . len () } fn split_at (self , index : usize) -> Option < [Self ; 2] > { self . split_at_mut_checked (index) . map (| (a , b) | [a , b]) } fn copy_from_slice_exact (self , src : & [u8]) -> Result < () > { self . copy_from_slice (src) ; Ok (()) } fn zero_out (self) -> Result < () > { self . fill (0) ; Ok (()) } }
};
}

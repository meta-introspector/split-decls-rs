// Generated macro for impl_74 (impl)
macro_rules! Depcrate_complex_lstmimpl_74 {
() => {
// Module: crate::complex::lstm
// Provides: {"impl_74"}
// Dependencies: {}
impl Iterator for LstmSegmenterIteratorUtf16 < '_ , '_ > { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { loop { self . pos += 1 ; if self . bies . next () ? || self . bies . len () == 0 { return Some (self . pos) ; } } } }
};
}

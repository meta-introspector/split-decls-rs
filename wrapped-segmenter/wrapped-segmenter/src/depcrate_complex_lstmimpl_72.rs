// Generated macro for impl_72 (impl)
macro_rules! Depcrate_complex_lstmimpl_72 {
() => {
// Module: crate::complex::lstm
// Provides: {"impl_72"}
// Dependencies: {}
impl Iterator for LstmSegmenterIterator < '_ , '_ > { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { loop { let is_e = self . bies . next () ? ; self . pos_utf8 += self . input [self . pos_utf8 ..] . chars () . next () ? . len_utf8 () ; if is_e || self . bies . len () == 0 { return Some (self . pos_utf8) ; } } } }
};
}

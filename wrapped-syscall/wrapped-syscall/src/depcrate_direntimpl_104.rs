// Generated macro for impl_104 (impl)
macro_rules! Depcrate_direntimpl_104 {
() => {
// Module: crate::dirent
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > Iterator for DirentIter < 'a > { type Item = Result < (& 'a DirentHeader , & 'a [u8]) , Invalid > ; fn next (& mut self) -> Option < Self :: Item > { if self . 0 . len () < size_of :: < DirentHeader > () { return None ; } let header = unsafe { & * (self . 0 . as_ptr () . cast :: < DirentHeader > ()) } ; if self . 0 . len () < usize :: from (header . record_len) { return Some (Err (Invalid)) ; } let (this , remaining) = self . 0 . split_at (usize :: from (header . record_len)) ; self . 0 = remaining ; let name_and_nul = & this [size_of :: < DirentHeader > () ..] ; let name = & name_and_nul [.. name_and_nul . len () - 1] ; Some (Ok ((header , name))) } }
};
}

// Generated macro for impl_761 (impl)
macro_rules! Depcrate_fsimpl_761 {
() => {
// Module: crate::fs
// Provides: {"impl_761"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < io :: Result < DirEntry > > { self . 0 . next () . map (| entry | entry . map (DirEntry)) } }
};
}

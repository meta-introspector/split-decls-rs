// Generated macro for impl_34 (impl)
macro_rules! Depcrate_archiveimpl_34 {
() => {
// Module: crate::archive
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , R : Read > Iterator for Entries < 'a , R > { type Item = io :: Result < Entry < 'a , R > > ; fn next (& mut self) -> Option < io :: Result < Entry < 'a , R > > > { self . fields . next () . map (| result | result . map (| e | EntryFields :: from (e) . into_entry ())) } }
};
}

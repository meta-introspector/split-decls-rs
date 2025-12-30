// Generated macro for impl_36 (impl)
macro_rules! Depcrate_archiveimpl_36 {
() => {
// Module: crate::archive
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a > Iterator for EntriesFields < 'a > { type Item = io :: Result < Entry < 'a , io :: Empty > > ; fn next (& mut self) -> Option < io :: Result < Entry < 'a , io :: Empty > > > { if self . done { None } else { match self . next_entry () { Ok (Some (e)) => Some (Ok (e)) , Ok (None) => { self . done = true ; None } Err (e) => { self . done = true ; Some (Err (e)) } } } } }
};
}

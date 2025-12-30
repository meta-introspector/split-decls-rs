// Generated macro for impl_102 (impl)
macro_rules! Depcrate_entryimpl_102 {
() => {
// Module: crate::entry
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > Read for EntryFields < 'a > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { loop { match self . data . get_mut (0) . map (| io | io . read (into)) { Some (Ok (0)) => { self . data . remove (0) ; } Some (r) => return r , None => return Ok (0) , } } } }
};
}

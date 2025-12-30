// Generated macro for impl_100 (impl)
macro_rules! Depcrate_entryimpl_100 {
() => {
// Module: crate::entry
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a , R : Read > Read for Entry < 'a , R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { self . fields . read (into) } }
};
}

// Generated macro for impl_103 (impl)
macro_rules! Depcrate_entryimpl_103 {
() => {
// Module: crate::entry
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a > Read for EntryIo < 'a > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { match * self { EntryIo :: Pad (ref mut io) => io . read (into) , EntryIo :: Data (ref mut io) => io . read (into) , } } }
};
}

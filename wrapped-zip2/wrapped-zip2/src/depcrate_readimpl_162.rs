// Generated macro for impl_162 (impl)
macro_rules! Depcrate_readimpl_162 {
() => {
// Module: crate::read
// Provides: {"impl_162"}
// Dependencies: {}
impl < R : Read > Read for SeekableTake < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let written = self . inner . take (self . length - self . current_offset) . read (buf) ? ; self . current_offset += written as u64 ; Ok (written) } }
};
}

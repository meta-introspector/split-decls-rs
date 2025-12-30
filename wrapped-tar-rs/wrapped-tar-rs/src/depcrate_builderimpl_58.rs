// Generated macro for impl_58 (impl)
macro_rules! Depcrate_builderimpl_58 {
() => {
// Module: crate::builder
// Provides: {"impl_58"}
// Dependencies: {}
impl Write for EntryWriter < '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let len = self . obj . write (buf) ? ; self . written += len as u64 ; Ok (len) } fn flush (& mut self) -> io :: Result < () > { self . obj . flush () } }
};
}

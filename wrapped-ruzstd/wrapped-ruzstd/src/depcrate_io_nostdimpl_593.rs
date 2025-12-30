// Generated macro for impl_593 (impl)
macro_rules! Depcrate_io_nostdimpl_593 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_593"}
// Dependencies: {}
impl < T > Write for & mut T where T : Write , { fn write (& mut self , buf : & [u8]) -> Result < usize , Error > { (* self) . write (buf) } fn flush (& mut self) -> Result < () , Error > { (* self) . flush () } }
};
}

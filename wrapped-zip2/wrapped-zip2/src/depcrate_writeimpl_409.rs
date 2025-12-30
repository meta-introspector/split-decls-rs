// Generated macro for impl_409 (impl)
macro_rules! Depcrate_writeimpl_409 {
() => {
// Module: crate::write
// Provides: {"impl_409"}
// Dependencies: {}
impl < W : Write > Write for StreamWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let bytes_written = self . inner . write (buf) ? ; self . bytes_written += bytes_written as u64 ; Ok (bytes_written) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}

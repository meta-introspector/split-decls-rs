// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < W : io :: Write > io :: Write for LossyStandardStream < W > { # [cfg (not (windows))] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . wtr . write (buf) } # [cfg (windows)] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if self . is_console { write_lossy_utf8 (& mut self . wtr , buf) } else { self . wtr . write (buf) } } fn flush (& mut self) -> io :: Result < () > { self . wtr . flush () } }
};
}

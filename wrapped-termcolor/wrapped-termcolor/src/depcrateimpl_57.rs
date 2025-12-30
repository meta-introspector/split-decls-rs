// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < W : io :: Write > io :: Write for Ansi < W > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . 0 . write_all (buf) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } }
};
}

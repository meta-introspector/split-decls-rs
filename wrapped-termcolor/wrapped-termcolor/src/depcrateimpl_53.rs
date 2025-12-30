// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < W : io :: Write > io :: Write for NoColor < W > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } }
};
}

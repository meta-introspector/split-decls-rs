// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl io :: Write for BufferedStandardStream { # [inline] fn write (& mut self , b : & [u8]) -> io :: Result < usize > { self . wtr . write (b) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . wtr . flush () } }
};
}

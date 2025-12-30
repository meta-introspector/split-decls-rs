// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl io :: Write for StandardStream { # [inline] fn write (& mut self , b : & [u8]) -> io :: Result < usize > { self . wtr . write (b) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . wtr . flush () } }
};
}

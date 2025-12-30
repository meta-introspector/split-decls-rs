// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a > io :: Write for StandardStreamLock < 'a > { # [inline] fn write (& mut self , b : & [u8]) -> io :: Result < usize > { self . wtr . write (b) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . wtr . flush () } }
};
}

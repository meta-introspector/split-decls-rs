// Generated macro for impl_101 (impl)
macro_rules! Depcrate_writerimpl_101 {
() => {
// Module: crate::writer
// Provides: {"impl_101"}
// Dependencies: {}
impl < W : io :: Write > io :: Write for LineWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . flush_line () ? ; self . wtr . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . flush_line () ? ; self . wtr . flush () } }
};
}

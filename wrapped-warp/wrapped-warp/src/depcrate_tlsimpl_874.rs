// Generated macro for impl_874 (impl)
macro_rules! Depcrate_tlsimpl_874 {
() => {
// Module: crate::tls
// Provides: {"impl_874"}
// Dependencies: {}
impl LazyFile { fn lazy_read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . file . is_none () { self . file = Some (File :: open (& self . path) ?) ; } self . file . as_mut () . unwrap () . read (buf) } }
};
}

// Generated macro for impl_875 (impl)
macro_rules! Depcrate_tlsimpl_875 {
() => {
// Module: crate::tls
// Provides: {"impl_875"}
// Dependencies: {}
impl Read for LazyFile { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . lazy_read (buf) . map_err (| err | { let kind = err . kind () ; io :: Error :: new (kind , format ! ("error reading file ({:?}): {}" , self . path . display () , err) ,) }) } }
};
}

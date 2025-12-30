// Generated macro for impl_221 (impl)
macro_rules! Depcrate_stream_bstrimpl_221 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_221"}
// Dependencies: {}
impl fmt :: Debug for BStr { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ! f . alternate () { write ! (f , "\"") ? ; } for byte in self . as_bytes () { let c = * byte as char ; write ! (f , "{}" , c . escape_debug ()) ? ; } if ! f . alternate () { write ! (f , "\"") ? ; } Ok (()) } }
};
}

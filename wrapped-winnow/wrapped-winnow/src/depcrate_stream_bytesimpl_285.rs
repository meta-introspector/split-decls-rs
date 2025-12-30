// Generated macro for impl_285 (impl)
macro_rules! Depcrate_stream_bytesimpl_285 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_285"}
// Dependencies: {}
impl fmt :: LowerHex for Bytes { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . as_bytes () { write ! (f , "{byte:0>2x}") ? ; } Ok (()) } }
};
}

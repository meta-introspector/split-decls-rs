// Generated macro for impl_286 (impl)
macro_rules! Depcrate_stream_bytesimpl_286 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_286"}
// Dependencies: {}
impl fmt :: UpperHex for Bytes { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , byte) in self . as_bytes () . iter () . enumerate () { if 0 < i { let absolute = (self . as_bytes () . as_ptr () as usize) + i ; if f . alternate () && absolute != 0 && absolute % 4 == 0 { write ! (f , "_") ? ; } } write ! (f , "{byte:0>2X}") ? ; } Ok (()) } }
};
}

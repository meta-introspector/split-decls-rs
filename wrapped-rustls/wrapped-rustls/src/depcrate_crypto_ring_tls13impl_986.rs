// Generated macro for impl_986 (impl)
macro_rules! Depcrate_crypto_ring_tls13impl_986 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"impl_986"}
// Dependencies: {}
impl HkdfExpander for RingHkdfExpander { fn expand_slice (& self , info : & [& [u8]] , output : & mut [u8]) -> Result < () , OutputLengthError > { self . prk . expand (info , Len (output . len ())) . and_then (| okm | okm . fill (output)) . map_err (| _ | OutputLengthError) } fn expand_block (& self , info : & [& [u8]]) -> OkmBlock { let mut buf = [0u8 ; OkmBlock :: MAX_LEN] ; let output = & mut buf [.. self . hash_len ()] ; self . prk . expand (info , Len (output . len ())) . and_then (| okm | okm . fill (output)) . unwrap () ; OkmBlock :: new (output) } fn hash_len (& self) -> usize { self . alg . len () } }
};
}

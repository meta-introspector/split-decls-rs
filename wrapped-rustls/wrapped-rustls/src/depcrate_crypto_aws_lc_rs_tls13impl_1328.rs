// Generated macro for impl_1328 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls13impl_1328 {
() => {
// Module: crate::crypto::aws_lc_rs::tls13
// Provides: {"impl_1328"}
// Dependencies: {}
impl HkdfExpander for AwsLcHkdfExpander { fn expand_slice (& self , info : & [& [u8]] , output : & mut [u8]) -> Result < () , OutputLengthError > { self . prk . expand (info , Len (output . len ())) . and_then (| okm | okm . fill (output)) . map_err (| _ | OutputLengthError) } fn expand_block (& self , info : & [& [u8]]) -> OkmBlock { let mut buf = [0u8 ; OkmBlock :: MAX_LEN] ; let output = & mut buf [.. self . hash_len ()] ; self . prk . expand (info , Len (output . len ())) . and_then (| okm | okm . fill (output)) . unwrap () ; OkmBlock :: new (output) } fn hash_len (& self) -> usize { self . alg . len () } }
};
}

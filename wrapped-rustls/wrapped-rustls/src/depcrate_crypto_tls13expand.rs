// Generated macro for expand (function)
macro_rules! Depcrate_crypto_tls13expand {
() => {
// Module: crate::crypto::tls13
// Provides: {"expand"}
// Dependencies: {}
# [doc = " `HKDF-Expand(PRK, info, L)` to construct any type from a byte array."] # [doc = ""] # [doc = " - `PRK` is the implicit key material represented by this instance."] # [doc = " - `L := N`; N is the size of the byte array."] # [doc = " - `info` is a slice of byte slices, which should be processed sequentially"] # [doc = "   (or concatenated if that is not possible)."] # [doc = ""] # [doc = " This is infallible, because the set of types (and therefore their length) is known"] # [doc = " at compile time."] pub fn expand < T , const N : usize > (expander : & dyn HkdfExpander , info : & [& [u8]]) -> T where T : From < [u8 ; N] > , { let mut output = [0u8 ; N] ; expander . expand_slice (info , & mut output) . expect ("expand type parameter T is too large") ; T :: from (output) }
};
}

// Generated macro for HkdfExpander (trait)
macro_rules! Depcrate_crypto_tls13HkdfExpander {
() => {
// Module: crate::crypto::tls13
// Provides: {"HkdfExpander"}
// Dependencies: {}
# [doc = " Implementation of `HKDF-Expand` with an implicitly stored and immutable `PRK`."] pub trait HkdfExpander : Send + Sync { # [doc = " `HKDF-Expand(PRK, info, L)` into a slice."] # [doc = ""] # [doc = " Where:"] # [doc = ""] # [doc = " - `PRK` is the implicit key material represented by this instance."] # [doc = " - `L` is `output.len()`."] # [doc = " - `info` is a slice of byte slices, which should be processed sequentially"] # [doc = "   (or concatenated if that is not possible)."] # [doc = ""] # [doc = " Returns `Err(OutputLengthError)` if `L` is larger than `255 * HashLen`."] # [doc = " Otherwise, writes to `output`."] fn expand_slice (& self , info : & [& [u8]] , output : & mut [u8]) -> Result < () , OutputLengthError > ; # [doc = " `HKDF-Expand(PRK, info, L=HashLen)` returned as a value."] # [doc = ""] # [doc = " - `PRK` is the implicit key material represented by this instance."] # [doc = " - `L := HashLen`."] # [doc = " - `info` is a slice of byte slices, which should be processed sequentially"] # [doc = "   (or concatenated if that is not possible)."] # [doc = ""] # [doc = " This is infallible, because by definition `OkmBlock` is always exactly"] # [doc = " `HashLen` bytes long."] fn expand_block (& self , info : & [& [u8]]) -> OkmBlock ; # [doc = " Return what `HashLen` is for this instance."] # [doc = ""] # [doc = " This must be no larger than [`OkmBlock::MAX_LEN`]."] fn hash_len (& self) -> usize ; }
};
}

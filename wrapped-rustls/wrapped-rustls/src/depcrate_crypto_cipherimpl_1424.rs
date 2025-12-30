// Generated macro for impl_1424 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1424 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1424"}
// Dependencies: {}
impl AeadKey { pub (crate) fn new (buf : & [u8]) -> Self { debug_assert ! (buf . len () <= Self :: MAX_LEN) ; let mut key = Self :: from ([0u8 ; Self :: MAX_LEN]) ; key . buf [.. buf . len ()] . copy_from_slice (buf) ; key . used = buf . len () ; key } pub (crate) fn with_length (self , len : usize) -> Self { assert ! (len <= self . used) ; Self { buf : self . buf , used : len , } } # [doc = " Largest possible AEAD key in the ciphersuites we support."] pub (crate) const MAX_LEN : usize = 32 ; }
};
}

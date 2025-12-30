// Generated macro for AeadTicketer (struct)
macro_rules! Depcrate_crypto_ring_ticketerAeadTicketer {
() => {
// Module: crate::crypto::ring::ticketer
// Provides: {"AeadTicketer"}
// Dependencies: {}
# [doc = " A [`TicketProducer`] implementation which can use any *ring* `aead::Algorithm`."] # [doc = ""] # [doc = " It does not enforce any lifetime constraint."] pub (super) struct AeadTicketer { alg : & 'static aead :: Algorithm , key : aead :: LessSafeKey , key_name : [u8 ; 16] , # [doc = " Tracks the largest ciphertext produced by `encrypt`, and"] # [doc = " uses it to early-reject `decrypt` queries that are too long."] # [doc = ""] # [doc = " Accepting excessively long ciphertexts means a \"Partitioning"] # [doc = " Oracle Attack\" (see <https://eprint.iacr.org/2020/1491.pdf>)"] # [doc = " can be more efficient, though also note that these are thought"] # [doc = " to be cryptographically hard if the key is full-entropy (as it"] # [doc = " is here)."] maximum_ciphertext_len : AtomicUsize , }
};
}

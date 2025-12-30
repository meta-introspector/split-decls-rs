// Generated macro for impl_1393 (impl)
macro_rules! Depcrate_crypto_cipher_outboundimpl_1393 {
() => {
// Module: crate::crypto::cipher::outbound
// Provides: {"impl_1393"}
// Dependencies: {}
impl PrefixedPayload { # [doc = " Create a new value with the given payload capacity."] # [doc = ""] # [doc = " (The actual capacity of the returned value will be at least `HEADER_SIZE + capacity`.)"] pub fn with_capacity (capacity : usize) -> Self { let mut prefixed_payload = Vec :: with_capacity (HEADER_SIZE + capacity) ; prefixed_payload . resize (HEADER_SIZE , 0) ; Self (prefixed_payload) } # [doc = " Append bytes from a slice."] pub fn extend_from_slice (& mut self , slice : & [u8]) { self . 0 . extend_from_slice (slice) } # [doc = " Append bytes from an `OutboundChunks`."] pub fn extend_from_chunks (& mut self , chunks : & OutboundChunks < '_ >) { chunks . copy_to_vec (& mut self . 0) } # [doc = " Truncate the payload to the given length (plus header)."] pub fn truncate (& mut self , len : usize) { self . 0 . truncate (len + HEADER_SIZE) } fn len (& self) -> usize { self . 0 . len () - HEADER_SIZE } }
};
}

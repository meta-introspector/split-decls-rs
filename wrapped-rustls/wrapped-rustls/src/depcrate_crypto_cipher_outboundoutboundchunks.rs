// Generated macro for OutboundChunks (enum)
macro_rules! Depcrate_crypto_cipher_outboundOutboundChunks {
() => {
// Module: crate::crypto::cipher::outbound
// Provides: {"OutboundChunks"}
// Dependencies: {}
# [doc = " A collection of borrowed plaintext slices."] # [doc = ""] # [doc = " Warning: OutboundChunks does not guarantee that the simplest variant is used."] # [doc = " Multiple can hold non fragmented or empty payloads."] # [non_exhaustive] # [derive (Debug , Clone)] pub enum OutboundChunks < 'a > { # [doc = " A single byte slice."] # [doc = ""] # [doc = " Contrary to `Multiple`, this uses a single pointer indirection"] Single (& 'a [u8]) , # [doc = " A collection of chunks (byte slices)."] Multiple { # [doc = " A collection of byte slices that hold the buffered data."] chunks : & 'a [& 'a [u8]] , # [doc = " The start cursor into the first chunk."] start : usize , # [doc = " The end cursor into the last chunk."] end : usize , } , }
};
}

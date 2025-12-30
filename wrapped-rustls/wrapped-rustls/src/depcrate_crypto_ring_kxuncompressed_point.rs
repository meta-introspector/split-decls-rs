// Generated macro for uncompressed_point (function)
macro_rules! Depcrate_crypto_ring_kxuncompressed_point {
() => {
// Module: crate::crypto::ring::kx
// Provides: {"uncompressed_point"}
// Dependencies: {}
fn uncompressed_point (point : & [u8]) -> bool { matches ! (point . first () , Some (0x04)) }
};
}

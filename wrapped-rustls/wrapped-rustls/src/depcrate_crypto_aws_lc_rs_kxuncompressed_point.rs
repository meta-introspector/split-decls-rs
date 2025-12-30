// Generated macro for uncompressed_point (function)
macro_rules! Depcrate_crypto_aws_lc_rs_kxuncompressed_point {
() => {
// Module: crate::crypto::aws_lc_rs::kx
// Provides: {"uncompressed_point"}
// Dependencies: {}
fn uncompressed_point (point : & [u8]) -> bool { matches ! (point . first () , Some (0x04)) }
};
}

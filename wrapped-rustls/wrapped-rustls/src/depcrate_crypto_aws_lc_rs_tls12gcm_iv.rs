// Generated macro for gcm_iv (function)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12gcm_iv {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"gcm_iv"}
// Dependencies: {}
fn gcm_iv (write_iv : & [u8] , explicit : & [u8]) -> Iv { debug_assert_eq ! (write_iv . len () , 4) ; debug_assert_eq ! (explicit . len () , 8) ; let mut iv = [0 ; NONCE_LEN] ; iv [.. 4] . copy_from_slice (write_iv) ; iv [4 ..] . copy_from_slice (explicit) ; Iv :: new (& iv) . expect ("IV length is NONCE_LEN, which is within MAX_LEN") }
};
}

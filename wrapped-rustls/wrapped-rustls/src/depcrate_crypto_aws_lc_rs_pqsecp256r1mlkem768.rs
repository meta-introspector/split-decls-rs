// Generated macro for SECP256R1MLKEM768 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_pqSECP256R1MLKEM768 {
() => {
// Module: crate::crypto::aws_lc_rs::pq
// Provides: {"SECP256R1MLKEM768"}
// Dependencies: {}
# [doc = " This is the [SECP256R1MLKEM768] key exchange."] # [doc = ""] # [doc = " [SECP256R1MLKEM768]: <https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/>"] pub static SECP256R1MLKEM768 : & dyn SupportedKxGroup = & hybrid :: Hybrid { classical : kx_group :: SECP256R1 , post_quantum : MLKEM768 , name : NamedGroup :: secp256r1MLKEM768 , layout : hybrid :: Layout { classical_share_len : SECP256R1_LEN , post_quantum_client_share_len : MLKEM768_ENCAP_LEN , post_quantum_server_share_len : MLKEM768_CIPHERTEXT_LEN , post_quantum_first : false , } , } ;
};
}

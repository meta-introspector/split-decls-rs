// Generated macro for X25519MLKEM768 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_pqX25519MLKEM768 {
() => {
// Module: crate::crypto::aws_lc_rs::pq
// Provides: {"X25519MLKEM768"}
// Dependencies: {}
# [doc = " This is the [X25519MLKEM768] key exchange."] # [doc = ""] # [doc = " [X25519MLKEM768]: <https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/>"] pub static X25519MLKEM768 : & dyn SupportedKxGroup = & hybrid :: Hybrid { classical : kx_group :: X25519 , post_quantum : MLKEM768 , name : NamedGroup :: X25519MLKEM768 , layout : hybrid :: Layout { classical_share_len : X25519_LEN , post_quantum_client_share_len : MLKEM768_ENCAP_LEN , post_quantum_server_share_len : MLKEM768_CIPHERTEXT_LEN , post_quantum_first : true , } , } ;
};
}

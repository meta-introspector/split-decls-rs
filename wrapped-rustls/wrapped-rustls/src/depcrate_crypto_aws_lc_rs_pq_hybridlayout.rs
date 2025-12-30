// Generated macro for Layout (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_hybridLayout {
() => {
// Module: crate::crypto::aws_lc_rs::pq::hybrid
// Provides: {"Layout"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] pub (crate) struct Layout { # [doc = " Length of classical key share."] pub (crate) classical_share_len : usize , # [doc = " Length of post-quantum key share sent by client"] pub (crate) post_quantum_client_share_len : usize , # [doc = " Length of post-quantum key share sent by server"] pub (crate) post_quantum_server_share_len : usize , # [doc = " Whether the post-quantum element comes first in shares and secrets."] # [doc = ""] # [doc = " For dismal and unprincipled reasons, SECP256R1MLKEM768 has the"] # [doc = " classical element first, while X25519MLKEM768 has it second."] pub (crate) post_quantum_first : bool , }
};
}

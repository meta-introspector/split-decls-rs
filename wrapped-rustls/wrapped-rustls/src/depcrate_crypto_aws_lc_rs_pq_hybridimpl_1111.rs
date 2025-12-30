// Generated macro for impl_1111 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_hybridimpl_1111 {
() => {
// Module: crate::crypto::aws_lc_rs::pq::hybrid
// Provides: {"impl_1111"}
// Dependencies: {}
impl Layout { fn split_received_client_share < 'a > (& self , share : & 'a [u8]) -> Option < (& 'a [u8] , & 'a [u8]) > { self . split (share , self . post_quantum_client_share_len) } fn split_received_server_share < 'a > (& self , share : & 'a [u8]) -> Option < (& 'a [u8] , & 'a [u8]) > { self . split (share , self . post_quantum_server_share_len) } # [doc = " Return the PQ and classical component of a key share."] fn split < 'a > (& self , share : & 'a [u8] , post_quantum_share_len : usize ,) -> Option < (& 'a [u8] , & 'a [u8]) > { if share . len () != self . classical_share_len + post_quantum_share_len { return None ; } Some (match self . post_quantum_first { true => { let (first_share , second_share) = share . split_at (post_quantum_share_len) ; (first_share , second_share) } false => { let (first_share , second_share) = share . split_at (self . classical_share_len) ; (second_share , first_share) } }) } fn concat (& self , post_quantum : & [u8] , classical : & [u8]) -> Vec < u8 > { match self . post_quantum_first { true => [post_quantum , classical] . concat () , false => [classical , post_quantum] . concat () , } } }
};
}

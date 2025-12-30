// Generated macro for impl_1108 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_hybridimpl_1108 {
() => {
// Module: crate::crypto::aws_lc_rs::pq::hybrid
// Provides: {"impl_1108"}
// Dependencies: {}
impl ActiveKeyExchange for ActiveHybrid { fn complete (self : Box < Self > , peer_pub_key : & [u8]) -> Result < SharedSecret , Error > { let (post_quantum_share , classical_share) = self . layout . split_received_server_share (peer_pub_key) . ok_or (INVALID_KEY_SHARE) ? ; let cl = self . classical . complete (classical_share) ? ; let pq = self . post_quantum . complete (post_quantum_share) ? ; let secret = self . layout . concat (pq . secret_bytes () , cl . secret_bytes ()) ; Ok (SharedSecret :: from (secret)) } fn pub_key (& self) -> & [u8] { & self . combined_pub_key } fn group (& self) -> NamedGroup { self . name } }
};
}

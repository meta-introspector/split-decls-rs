// Generated macro for impl_1122 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_mlkemimpl_1122 {
() => {
// Module: crate::crypto::aws_lc_rs::pq::mlkem
// Provides: {"impl_1122"}
// Dependencies: {}
impl ActiveKeyExchange for Active { fn complete (self : Box < Self > , peer_pub_key : & [u8]) -> Result < SharedSecret , Error > { let shared_secret = self . decaps_key . decapsulate (peer_pub_key . into ()) . map_err (| _ | INVALID_KEY_SHARE) ? ; Ok (SharedSecret :: from (shared_secret . as_ref ())) } fn pub_key (& self) -> & [u8] { & self . encaps_key_bytes } fn group (& self) -> NamedGroup { NamedGroup :: MLKEM768 } }
};
}

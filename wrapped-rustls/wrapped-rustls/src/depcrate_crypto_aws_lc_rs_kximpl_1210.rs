// Generated macro for impl_1210 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_kximpl_1210 {
() => {
// Module: crate::crypto::aws_lc_rs::kx
// Provides: {"impl_1210"}
// Dependencies: {}
impl ActiveKeyExchange for KeyExchange { # [doc = " Completes the key exchange, given the peer's public key."] fn complete (self : Box < Self > , peer : & [u8]) -> Result < SharedSecret , Error > { if ! (self . pub_key_validator) (peer) { return Err (PeerMisbehaved :: InvalidKeyShare . into ()) ; } let peer_key = agreement :: UnparsedPublicKey :: new (self . agreement_algorithm , peer) ; super :: ring_shim :: agree_ephemeral (self . priv_key , & peer_key) . map_err (| _ | PeerMisbehaved :: InvalidKeyShare . into ()) } # [doc = " Return the group being used."] fn group (& self) -> NamedGroup { self . name } # [doc = " Return the public key being used."] fn pub_key (& self) -> & [u8] { self . pub_key . as_ref () } }
};
}

// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl crypto :: ActiveKeyExchange for ActiveKeyExchange { fn complete (self : Box < Self > , peer : & [u8]) -> Result < crypto :: SharedSecret , Error > { match peer { KX_PEER_SHARE => Ok (crypto :: SharedSecret :: from (KX_SHARED_SECRET)) , _ => Err (Error :: from (PeerMisbehaved :: InvalidKeyShare)) , } } fn pub_key (& self) -> & [u8] { KX_PEER_SHARE } fn group (& self) -> NamedGroup { NamedGroup :: from (0xfe00) } }
};
}

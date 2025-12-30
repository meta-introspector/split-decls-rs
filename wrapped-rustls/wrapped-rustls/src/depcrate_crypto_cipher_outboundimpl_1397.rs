// Generated macro for impl_1397 (impl)
macro_rules! Depcrate_crypto_cipher_outboundimpl_1397 {
() => {
// Module: crate::crypto::cipher::outbound
// Provides: {"impl_1397"}
// Dependencies: {}
impl From < & [u8] > for PrefixedPayload { fn from (content : & [u8]) -> Self { let mut payload = Vec :: with_capacity (HEADER_SIZE + content . len ()) ; payload . extend (& [0u8 ; HEADER_SIZE]) ; payload . extend (content) ; Self (payload) } }
};
}

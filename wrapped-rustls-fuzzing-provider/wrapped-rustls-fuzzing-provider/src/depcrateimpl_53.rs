// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl MessageEncrypter for Tls12Cipher { fn encrypt (& mut self , m : OutboundPlainMessage < '_ > , seq : u64 ,) -> Result < OutboundOpaqueMessage , Error > { let total_len = self . encrypted_payload_len (m . payload . len ()) ; let mut payload = PrefixedPayload :: with_capacity (total_len) ; payload . extend_from_chunks (& m . payload) ; for (p , mask) in payload . as_mut () . iter_mut () . zip (AEAD_MASK . iter () . cycle ()) { * p ^= * mask ; } payload . extend_from_slice (& seq . to_be_bytes ()) ; payload . extend_from_slice (AEAD_TAG) ; Ok (OutboundOpaqueMessage { typ : m . typ , version : m . version , payload , }) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len + AEAD_OVERHEAD } }
};
}

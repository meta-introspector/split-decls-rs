// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl MessageDecrypter for Tls13Cipher { fn decrypt < 'a > (& mut self , mut m : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > { let payload = & mut m . payload ; let mut expected_tag = vec ! [] ; expected_tag . extend_from_slice (& seq . to_be_bytes ()) ; expected_tag . extend_from_slice (AEAD_TAG) ; if payload . len () < AEAD_OVERHEAD || payload . as_ref () [payload . len () - AEAD_OVERHEAD ..] != expected_tag { return Err (Error :: DecryptError) ; } payload . truncate (payload . len () - AEAD_OVERHEAD) ; for (p , mask) in payload . as_mut () . iter_mut () . zip (AEAD_MASK . iter () . cycle ()) { * p ^= * mask ; } m . into_tls13_unpadded_message () } }
};
}

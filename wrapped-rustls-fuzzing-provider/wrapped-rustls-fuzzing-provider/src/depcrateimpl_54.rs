// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl MessageDecrypter for Tls12Cipher { fn decrypt < 'a > (& mut self , mut m : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > { let payload = & mut m . payload ; let mut expected_tag = vec ! [] ; expected_tag . extend_from_slice (& seq . to_be_bytes ()) ; expected_tag . extend_from_slice (AEAD_TAG) ; if payload . len () < AEAD_OVERHEAD || payload . as_ref () [payload . len () - AEAD_OVERHEAD ..] != expected_tag { return Err (Error :: DecryptError) ; } payload . truncate (payload . len () - AEAD_OVERHEAD) ; for (p , mask) in payload . as_mut () . iter_mut () . zip (AEAD_MASK . iter () . cycle ()) { * p ^= * mask ; } Ok (m . into_plain_message ()) } }
};
}

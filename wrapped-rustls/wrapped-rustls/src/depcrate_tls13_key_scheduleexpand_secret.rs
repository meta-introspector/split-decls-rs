// Generated macro for expand_secret (function)
macro_rules! Depcrate_tls13_key_scheduleexpand_secret {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"expand_secret"}
// Dependencies: {}
fn expand_secret (secret : & OkmBlock , hkdf : & 'static dyn Hkdf , aead_key_len : usize , iv_len : usize ,) -> (AeadKey , Iv) { let expander = hkdf . expander_for_okm (secret) ; (hkdf_expand_label_aead_key (expander . as_ref () , aead_key_len , b"key" , & []) , derive_traffic_iv (expander . as_ref () , iv_len) ,) }
};
}

// Generated macro for hkdf_expand_label_aead_key (function)
macro_rules! Depcrate_tls13_key_schedulehkdf_expand_label_aead_key {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"hkdf_expand_label_aead_key"}
// Dependencies: {}
# [doc = " [HKDF-Expand-Label] where the output is an AEAD key."] pub (crate) fn hkdf_expand_label_aead_key (expander : & dyn HkdfExpander , key_len : usize , label : & [u8] , context : & [u8] ,) -> AeadKey { hkdf_expand_label_inner (expander , label , context , key_len , | e , info | { let key : AeadKey = expand (e , info) ; key . with_length (key_len) }) }
};
}

// Generated macro for derive_traffic_key (function)
macro_rules! Depcrate_tls13_key_schedulederive_traffic_key {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"derive_traffic_key"}
// Dependencies: {}
# [doc = " [HKDF-Expand-Label] where the output is an AEAD key."] # [doc = ""] # [doc = " [HKDF-Expand-Label]: <https://www.rfc-editor.org/rfc/rfc8446#section-7.1>"] pub (crate) fn derive_traffic_key (expander : & dyn HkdfExpander , aead_alg : & dyn Tls13AeadAlgorithm ,) -> AeadKey { hkdf_expand_label_aead_key (expander , aead_alg . key_len () , b"key" , & []) }
};
}

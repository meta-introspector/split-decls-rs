// Generated macro for derive_traffic_iv (function)
macro_rules! Depcrate_tls13_key_schedulederive_traffic_iv {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"derive_traffic_iv"}
// Dependencies: {}
# [doc = " [HKDF-Expand-Label] where the output is an IV with a specified length."] # [doc = ""] # [doc = " [HKDF-Expand-Label]: <https://www.rfc-editor.org/rfc/rfc8446#section-7.1>"] pub (crate) fn derive_traffic_iv (expander : & dyn HkdfExpander , iv_len : usize) -> Iv { hkdf_expand_label_iv (expander , b"iv" , & [] , iv_len) }
};
}

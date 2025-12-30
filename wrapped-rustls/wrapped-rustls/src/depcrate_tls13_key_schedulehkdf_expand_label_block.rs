// Generated macro for hkdf_expand_label_block (function)
macro_rules! Depcrate_tls13_key_schedulehkdf_expand_label_block {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"hkdf_expand_label_block"}
// Dependencies: {}
# [doc = " [HKDF-Expand-Label] where the output is one block in size."] pub (crate) fn hkdf_expand_label_block (expander : & dyn HkdfExpander , label : & [u8] , context : & [u8] ,) -> OkmBlock { hkdf_expand_label_inner (expander , label , context , expander . hash_len () , | e , info | { e . expand_block (info) }) }
};
}

// Generated macro for hkdf_expand_label (function)
macro_rules! Depcrate_tls13_key_schedulehkdf_expand_label {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"hkdf_expand_label"}
// Dependencies: {}
# [doc = " [HKDF-Expand-Label] where the output length is a compile-time constant, and therefore"] # [doc = " it is infallible."] # [doc = ""] # [doc = " [HKDF-Expand-Label]: <https://www.rfc-editor.org/rfc/rfc8446#section-7.1>"] pub (crate) fn hkdf_expand_label < T : From < [u8 ; N] > , const N : usize > (expander : & dyn HkdfExpander , label : & [u8] , context : & [u8] ,) -> T { hkdf_expand_label_inner (expander , label , context , N , | e , info | expand (e , info)) }
};
}

// Generated macro for hkdf_expand_label_slice (function)
macro_rules! Depcrate_tls13_key_schedulehkdf_expand_label_slice {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"hkdf_expand_label_slice"}
// Dependencies: {}
# [doc = " [HKDF-Expand-Label] where the output is a slice."] # [doc = ""] # [doc = " This can fail because HKDF-Expand is limited in its maximum output length."] fn hkdf_expand_label_slice (expander : & dyn HkdfExpander , label : & [u8] , context : & [u8] , output : & mut [u8] ,) -> Result < () , OutputLengthError > { hkdf_expand_label_inner (expander , label , context , output . len () , | e , info | { e . expand_slice (info , output) }) }
};
}

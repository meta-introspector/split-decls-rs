// Generated macro for hkdf_expand_label_iv (function)
macro_rules! Depcrate_tls13_key_schedulehkdf_expand_label_iv {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"hkdf_expand_label_iv"}
// Dependencies: {}
# [doc = " [HKDF-Expand-Label] where the output is an IV."] pub (crate) fn hkdf_expand_label_iv (expander : & dyn HkdfExpander , label : & [u8] , context : & [u8] , iv_len : usize ,) -> Iv { hkdf_expand_label_inner (expander , label , context , iv_len , | e , info | { let mut buf = [0u8 ; Iv :: MAX_LEN] ; e . expand_slice (info , & mut buf [.. iv_len]) . unwrap () ; Iv :: new (& buf [.. iv_len]) . expect ("IV length from cipher suite must be within MAX_LEN") }) }
};
}

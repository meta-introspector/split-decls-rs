// Generated macro for hkdf_expand_label_inner (function)
macro_rules! Depcrate_tls13_key_schedulehkdf_expand_label_inner {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"hkdf_expand_label_inner"}
// Dependencies: {}
fn hkdf_expand_label_inner < F , T > (expander : & dyn HkdfExpander , label : & [u8] , context : & [u8] , n : usize , f : F ,) -> T where F : FnOnce (& dyn HkdfExpander , & [& [u8]]) -> T , { const LABEL_PREFIX : & [u8] = b"tls13 " ; let output_len = u16 :: to_be_bytes (n as u16) ; let label_len = u8 :: to_be_bytes ((LABEL_PREFIX . len () + label . len ()) as u8) ; let context_len = u8 :: to_be_bytes (context . len () as u8) ; let info = & [& output_len [..] , & label_len [..] , LABEL_PREFIX , label , & context_len [..] , context ,] ; f (expander , info) }
};
}

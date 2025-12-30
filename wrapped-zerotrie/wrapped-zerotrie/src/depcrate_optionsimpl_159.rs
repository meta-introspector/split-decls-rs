// Generated macro for impl_159 (impl)
macro_rules! Depcrate_optionsimpl_159 {
() => {
// Module: crate::options
// Provides: {"impl_159"}
// Dependencies: {}
impl ZeroTrieBuilderOptions { # [cfg (feature = "serde")] pub (crate) const fn to_u8_flags (self) -> u8 { self . phf_mode . to_u8_flag () | self . ascii_mode . to_u8_flag () | self . capacity_mode . to_u8_flag () | self . case_sensitivity . to_u8_flag () } }
};
}

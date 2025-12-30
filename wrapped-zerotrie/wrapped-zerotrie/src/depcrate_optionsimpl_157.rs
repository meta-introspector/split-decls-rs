// Generated macro for impl_157 (impl)
macro_rules! Depcrate_optionsimpl_157 {
() => {
// Module: crate::options
// Provides: {"impl_157"}
// Dependencies: {}
impl CaseSensitivity { # [cfg (feature = "serde")] const fn to_u8_flag (self) -> u8 { match self { Self :: Sensitive => 0 , Self :: IgnoreCase => 0x8 , } } }
};
}

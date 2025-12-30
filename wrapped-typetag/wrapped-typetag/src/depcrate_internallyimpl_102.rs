// Generated macro for impl_102 (impl)
macro_rules! Depcrate_internallyimpl_102 {
() => {
// Module: crate::internally
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'de , A > MapWithStringKeys < A > where A : MapAccess < 'de > , { fn try_default_key (& mut self) -> Result < () , A :: Error > { self . map . next_key_seed (DefaultKey) ? . ok_or_else (| | de :: Error :: missing_field (DEFAULT_KEY)) } }
};
}

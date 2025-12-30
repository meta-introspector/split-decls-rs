// Generated macro for impl_195 (impl)
macro_rules! Depcrate_namesimpl_195 {
() => {
// Module: crate::names
// Provides: {"impl_195"}
// Dependencies: {}
impl NameTag { fn with_codepoint (& self , cp : u32) -> u64 { use self :: NameTag :: * ; match * self { Explicit => (1 << 33) | (cp as u64) , Alias => (1 << 34) | (cp as u64) , Hangul => (1 << 35) | (cp as u64) , Ideograph => (1 << 36) | (cp as u64) , } } }
};
}

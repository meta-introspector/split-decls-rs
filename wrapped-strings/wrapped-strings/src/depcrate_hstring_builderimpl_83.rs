// Generated macro for impl_83 (impl)
macro_rules! Depcrate_hstring_builderimpl_83 {
() => {
// Module: crate::hstring_builder
// Provides: {"impl_83"}
// Dependencies: {}
impl core :: ops :: DerefMut for HStringBuilder { fn deref_mut (& mut self) -> & mut [u16] { if let Some (header) = self . as_header () { unsafe { core :: slice :: from_raw_parts_mut (header . data , header . len as usize) } } else { & mut [] } } }
};
}

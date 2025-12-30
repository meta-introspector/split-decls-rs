// Generated macro for impl_82 (impl)
macro_rules! Depcrate_hstring_builderimpl_82 {
() => {
// Module: crate::hstring_builder
// Provides: {"impl_82"}
// Dependencies: {}
impl core :: ops :: Deref for HStringBuilder { type Target = [u16] ; fn deref (& self) -> & [u16] { if let Some (header) = self . as_header () { unsafe { core :: slice :: from_raw_parts (header . data , header . len as usize) } } else { & [] } } }
};
}

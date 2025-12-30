// Generated macro for impl_332 (impl)
macro_rules! Depcrate__privateimpl_332 {
() => {
// Module: crate::_private
// Provides: {"impl_332"}
// Dependencies: {}
impl < T : Serialize > MaybeSerializeWrapper < T > { pub fn maybe_to_value (self) -> Option < Value > { serde_json :: value :: to_value (self . 0) . ok () } }
};
}

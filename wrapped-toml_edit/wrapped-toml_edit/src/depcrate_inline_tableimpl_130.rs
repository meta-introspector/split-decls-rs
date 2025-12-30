// Generated macro for impl_130 (impl)
macro_rules! Depcrate_inline_tableimpl_130 {
() => {
// Module: crate::inline_table
// Provides: {"impl_130"}
// Dependencies: {}
impl < 's > IntoIterator for & 's InlineTable { type Item = (& 's str , & 's Value) ; type IntoIter = InlineTableIter < 's > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}

// Generated macro for impl_129 (impl)
macro_rules! Depcrate_inline_tableimpl_129 {
() => {
// Module: crate::inline_table
// Provides: {"impl_129"}
// Dependencies: {}
impl IntoIterator for InlineTable { type Item = (String , Value) ; type IntoIter = InlineTableIntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . items . into_iter () . filter (| (_ , value) | value . is_value ()) . map (| (key , value) | (key . into () , value . into_value () . unwrap ())) ,) } }
};
}

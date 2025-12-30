// Generated macro for impl_318 (impl)
macro_rules! Depcrate_tableimpl_318 {
() => {
// Module: crate::table
// Provides: {"impl_318"}
// Dependencies: {}
impl IntoIterator for Table { type Item = (String , Item) ; type IntoIter = IntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . items . into_iter () . map (| (k , value) | (k . into () , value))) } }
};
}

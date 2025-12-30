// Generated macro for impl_17 (impl)
macro_rules! Depcrate_arrayimpl_17 {
() => {
// Module: crate::array
// Provides: {"impl_17"}
// Dependencies: {}
impl IntoIterator for Array { type Item = Value ; type IntoIter = ArrayIntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . values . into_iter () . filter (| v | v . is_value ()) . map (| v | v . into_value () . unwrap ()) ,) } }
};
}

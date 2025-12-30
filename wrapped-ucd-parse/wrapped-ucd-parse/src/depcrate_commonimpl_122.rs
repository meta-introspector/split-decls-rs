// Generated macro for impl_122 (impl)
macro_rules! Depcrate_commonimpl_122 {
() => {
// Module: crate::common
// Provides: {"impl_122"}
// Dependencies: {}
impl IntoIterator for Codepoints { type IntoIter = CodepointIter ; type Item = Codepoint ; fn into_iter (self) -> CodepointIter { match self { Codepoints :: Single (x) => x . into_iter () , Codepoints :: Range (x) => x . into_iter () , } } }
};
}

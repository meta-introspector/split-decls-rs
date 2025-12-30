// Generated macro for impl_137 (impl)
macro_rules! Depcrate_commonimpl_137 {
() => {
// Module: crate::common
// Provides: {"impl_137"}
// Dependencies: {}
impl IntoIterator for Codepoint { type IntoIter = CodepointIter ; type Item = Codepoint ; fn into_iter (self) -> CodepointIter { let range = CodepointRange { start : self , end : self } ; CodepointIter { next : self . value () , range } } }
};
}

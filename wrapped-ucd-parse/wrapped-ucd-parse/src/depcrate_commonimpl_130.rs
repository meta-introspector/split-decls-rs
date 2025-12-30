// Generated macro for impl_130 (impl)
macro_rules! Depcrate_commonimpl_130 {
() => {
// Module: crate::common
// Provides: {"impl_130"}
// Dependencies: {}
impl IntoIterator for CodepointRange { type IntoIter = CodepointIter ; type Item = Codepoint ; fn into_iter (self) -> CodepointIter { CodepointIter { next : self . start . value () , range : self } } }
};
}

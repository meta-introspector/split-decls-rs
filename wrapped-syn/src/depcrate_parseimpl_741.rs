// Generated macro for impl_741 (impl)
macro_rules! Depcrate_parseimpl_741 {
() => {
// Module: crate::parse
// Provides: {"impl_741"}
// Dependencies: {}
impl Clone for Unexpected { fn clone (& self) -> Self { match self { Unexpected :: None => Unexpected :: None , Unexpected :: Some (span , delimiter) => Unexpected :: Some (* span , * delimiter) , Unexpected :: Chain (next) => Unexpected :: Chain (next . clone ()) , } } }
};
}

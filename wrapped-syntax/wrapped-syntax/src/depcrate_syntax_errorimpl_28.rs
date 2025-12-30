// Generated macro for impl_28 (impl)
macro_rules! Depcrate_syntax_errorimpl_28 {
() => {
// Module: crate::syntax_error
// Provides: {"impl_28"}
// Dependencies: {}
impl SyntaxError { pub fn new (message : impl Into < String > , range : TextRange) -> Self { Self (message . into () , range) } pub fn new_at_offset (message : impl Into < String > , offset : TextSize) -> Self { Self (message . into () , TextRange :: empty (offset)) } pub fn range (& self) -> TextRange { self . 1 } pub fn with_range (mut self , range : TextRange) -> Self { self . 1 = range ; self } }
};
}

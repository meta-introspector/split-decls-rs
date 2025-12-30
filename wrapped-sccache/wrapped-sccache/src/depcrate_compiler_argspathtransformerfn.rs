// Generated macro for PathTransformerFn (type)
macro_rules! Depcrate_compiler_argsPathTransformerFn {
() => {
// Module: crate::compiler::args
// Provides: {"PathTransformerFn"}
// Dependencies: {}
pub type PathTransformerFn < 'a > = & 'a mut dyn FnMut (& Path) -> Option < String > ;
};
}

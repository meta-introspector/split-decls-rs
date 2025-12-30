// Generated macro for impl_191 (impl)
macro_rules! Depcrate_syntax_editorimpl_191 {
() => {
// Module: crate::syntax_editor
// Provides: {"impl_191"}
// Dependencies: {}
impl Position { pub (crate) fn parent (& self) -> SyntaxNode { self . place () . 0 } pub (crate) fn place (& self) -> (SyntaxNode , usize) { match & self . repr { PositionRepr :: FirstChild (parent) => (parent . clone () , 0) , PositionRepr :: After (child) => (child . parent () . unwrap () , child . index () + 1) , } } }
};
}

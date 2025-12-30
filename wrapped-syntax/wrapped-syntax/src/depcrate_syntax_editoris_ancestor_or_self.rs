// Generated macro for is_ancestor_or_self (function)
macro_rules! Depcrate_syntax_editoris_ancestor_or_self {
() => {
// Module: crate::syntax_editor
// Provides: {"is_ancestor_or_self"}
// Dependencies: {}
fn is_ancestor_or_self (node : & SyntaxNode , ancestor : & SyntaxNode) -> bool { node == ancestor || node . ancestors () . any (| it | & it == ancestor) }
};
}

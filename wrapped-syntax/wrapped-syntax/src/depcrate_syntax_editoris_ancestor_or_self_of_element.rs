// Generated macro for is_ancestor_or_self_of_element (function)
macro_rules! Depcrate_syntax_editoris_ancestor_or_self_of_element {
() => {
// Module: crate::syntax_editor
// Provides: {"is_ancestor_or_self_of_element"}
// Dependencies: {}
fn is_ancestor_or_self_of_element (node : & SyntaxElement , ancestor : & SyntaxNode) -> bool { matches ! (node , SyntaxElement :: Node (node) if node == ancestor) || node . ancestors () . any (| it | & it == ancestor) }
};
}

// Generated macro for find_node_at_range (function)
macro_rules! Depcrate_algofind_node_at_range {
() => {
// Module: crate::algo
// Provides: {"find_node_at_range"}
// Dependencies: {}
pub fn find_node_at_range < N : AstNode > (syntax : & SyntaxNode , range : TextRange) -> Option < N > { syntax . covering_element (range) . ancestors () . find_map (N :: cast) }
};
}

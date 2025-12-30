// Generated macro for has_errors (function)
macro_rules! Depcrate_algohas_errors {
() => {
// Module: crate::algo
// Provides: {"has_errors"}
// Dependencies: {}
pub fn has_errors (node : & SyntaxNode) -> bool { node . children () . any (| it | it . kind () == SyntaxKind :: ERROR) }
};
}

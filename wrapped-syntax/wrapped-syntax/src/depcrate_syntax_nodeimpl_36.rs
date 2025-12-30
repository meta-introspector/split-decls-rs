// Generated macro for impl_36 (impl)
macro_rules! Depcrate_syntax_nodeimpl_36 {
() => {
// Module: crate::syntax_node
// Provides: {"impl_36"}
// Dependencies: {}
impl Language for RustLanguage { type Kind = SyntaxKind ; fn kind_from_raw (raw : rowan :: SyntaxKind) -> SyntaxKind { SyntaxKind :: from (raw . 0) } fn kind_to_raw (kind : SyntaxKind) -> rowan :: SyntaxKind { rowan :: SyntaxKind (kind . into ()) } }
};
}

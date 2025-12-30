// Generated macro for support (module)
macro_rules! Depcrate_astsupport {
() => {
// Module: crate::ast
// Provides: {"support"}
// Dependencies: {}
mod support { use super :: { AstChildren , AstNode , SyntaxKind , SyntaxNode , SyntaxToken } ; # [inline] pub (super) fn child < N : AstNode > (parent : & SyntaxNode) -> Option < N > { parent . children () . find_map (N :: cast) } # [inline] pub (super) fn children < N : AstNode > (parent : & SyntaxNode) -> AstChildren < N > { AstChildren :: new (parent) } # [inline] pub (super) fn token (parent : & SyntaxNode , kind : SyntaxKind) -> Option < SyntaxToken > { parent . children_with_tokens () . filter_map (| it | it . into_token ()) . find (| it | it . kind () == kind) } }
};
}

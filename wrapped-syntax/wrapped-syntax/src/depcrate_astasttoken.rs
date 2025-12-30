// Generated macro for AstToken (trait)
macro_rules! Depcrate_astAstToken {
() => {
// Module: crate::ast
// Provides: {"AstToken"}
// Dependencies: {}
# [doc = " Like `AstNode`, but wraps tokens rather than interior nodes."] pub trait AstToken { fn can_cast (token : SyntaxKind) -> bool where Self : Sized ; fn cast (syntax : SyntaxToken) -> Option < Self > where Self : Sized ; fn syntax (& self) -> & SyntaxToken ; fn text (& self) -> & str { self . syntax () . text () } }
};
}

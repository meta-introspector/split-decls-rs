// Generated macro for AstChildren (struct)
macro_rules! Depcrate_astAstChildren {
() => {
// Module: crate::ast
// Provides: {"AstChildren"}
// Dependencies: {}
# [doc = " An iterator over `SyntaxNode` children of a particular AST type."] # [derive (Debug , Clone)] pub struct AstChildren < N > { inner : SyntaxNodeChildren , ph : PhantomData < N > , }
};
}

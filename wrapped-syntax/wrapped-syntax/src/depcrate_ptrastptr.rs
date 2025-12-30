// Generated macro for AstPtr (struct)
macro_rules! Depcrate_ptrAstPtr {
() => {
// Module: crate::ptr
// Provides: {"AstPtr"}
// Dependencies: {}
# [doc = " Like `SyntaxNodePtr`, but remembers the type of node."] pub struct AstPtr < N : AstNode > { raw : SyntaxNodePtr , _ty : PhantomData < fn () -> N > , }
};
}

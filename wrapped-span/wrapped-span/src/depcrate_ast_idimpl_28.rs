// Generated macro for impl_28 (impl)
macro_rules! Depcrate_ast_idimpl_28 {
() => {
// Module: crate::ast_id
// Provides: {"impl_28"}
// Dependencies: {}
impl < N > FileAstId < N > { # [inline] pub fn upcast < M : AstIdNode > (self) -> FileAstId < M > where N : Into < M > , { FileAstId { raw : self . raw , _marker : PhantomData } } # [inline] pub fn erase (self) -> ErasedFileAstId { self . raw } }
};
}

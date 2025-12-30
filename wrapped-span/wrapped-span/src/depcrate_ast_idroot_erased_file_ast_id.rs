// Generated macro for ROOT_ERASED_FILE_AST_ID (const)
macro_rules! Depcrate_ast_idROOT_ERASED_FILE_AST_ID {
() => {
// Module: crate::ast_id
// Provides: {"ROOT_ERASED_FILE_AST_ID"}
// Dependencies: {}
# [doc = " The root ast id always points to the encompassing file, using this in spans is discouraged as"] # [doc = " any range relative to it will be effectively absolute, ruining the entire point of anchored"] # [doc = " relative text ranges."] pub const ROOT_ERASED_FILE_AST_ID : ErasedFileAstId = ErasedFileAstId (pack_hash_index_and_kind (0 , 0 , ErasedFileAstIdKind :: Root as u32)) ;
};
}

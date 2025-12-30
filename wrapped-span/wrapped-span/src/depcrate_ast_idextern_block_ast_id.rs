// Generated macro for extern_block_ast_id (function)
macro_rules! Depcrate_ast_idextern_block_ast_id {
() => {
// Module: crate::ast_id
// Provides: {"extern_block_ast_id"}
// Dependencies: {}
fn extern_block_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap ,) -> Option < ErasedFileAstId > { if ast :: ExternBlock :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: ExternBlock , ())) } else { None } }
};
}

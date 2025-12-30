// Generated macro for use_ast_id (function)
macro_rules! Depcrate_ast_iduse_ast_id {
() => {
// Module: crate::ast_id
// Provides: {"use_ast_id"}
// Dependencies: {}
fn use_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap ,) -> Option < ErasedFileAstId > { if ast :: Use :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: Use , ())) } else { None } }
};
}

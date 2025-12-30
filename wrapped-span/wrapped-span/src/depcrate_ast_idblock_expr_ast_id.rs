// Generated macro for block_expr_ast_id (function)
macro_rules! Depcrate_ast_idblock_expr_ast_id {
() => {
// Module: crate::ast_id
// Provides: {"block_expr_ast_id"}
// Dependencies: {}
fn block_expr_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap , parent : Option < & ErasedFileAstId > ,) -> Option < ErasedFileAstId > { if ast :: BlockExpr :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: BlockExpr , BlockExprFileAstId { parent : parent . copied () } ,) ,) } else { None } }
};
}

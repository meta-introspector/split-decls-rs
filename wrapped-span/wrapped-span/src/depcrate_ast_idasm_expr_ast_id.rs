// Generated macro for asm_expr_ast_id (function)
macro_rules! Depcrate_ast_idasm_expr_ast_id {
() => {
// Module: crate::ast_id
// Provides: {"asm_expr_ast_id"}
// Dependencies: {}
fn asm_expr_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap ,) -> Option < ErasedFileAstId > { if ast :: AsmExpr :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: AsmExpr , ())) } else { None } }
};
}

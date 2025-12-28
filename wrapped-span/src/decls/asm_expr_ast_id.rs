macro_rules! deps {
    () => {
        ErasedFileAstId!();
        ErasedFileAstIdKind!();
        ErasedAstIdNextIndexMap!();
    };
}

macro_rules! asm_expr_ast_id {
    () => {
        deps!();
        fn asm_expr_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap ,) -> Option < ErasedFileAstId > { if ast :: AsmExpr :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: AsmExpr , ())) } else { None } }
    };
}

asm_expr_ast_id!()
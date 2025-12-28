macro_rules! deps {
    () => {
        BlockExprFileAstId!();
        ErasedFileAstIdKind!();
        ErasedFileAstId!();
        ErasedAstIdNextIndexMap!();
    };
}

macro_rules! block_expr_ast_id {
    () => {
        deps!();
        fn block_expr_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap , parent : Option < & ErasedFileAstId > ,) -> Option < ErasedFileAstId > { if ast :: BlockExpr :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: BlockExpr , BlockExprFileAstId { parent : parent . copied () } ,) ,) } else { None } }
    };
}

block_expr_ast_id!()
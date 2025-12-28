macro_rules! deps {
    () => {
        ErasedAstIdNextIndexMap!();
        ErasedFileAstId!();
        ErasedFileAstIdKind!();
    };
}

macro_rules! extern_block_ast_id {
    () => {
        deps!();
        fn extern_block_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap ,) -> Option < ErasedFileAstId > { if ast :: ExternBlock :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: ExternBlock , ())) } else { None } }
    };
}

extern_block_ast_id!()
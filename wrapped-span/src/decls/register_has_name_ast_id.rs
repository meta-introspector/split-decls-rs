macro_rules! deps {
    () => {
        ErasedAstIdNextIndexMap!();
        ErasedFileAstId!();
        ErasedHasNameFileAstId!();
        AstIdNode!();
        ErasedFileAstIdKind!();
    };
}

macro_rules! register_has_name_ast_id {
    () => {
        deps!();
        macro_rules ! register_has_name_ast_id { (impl $ AstIdNode : ident for $ ($ ident : ident = $ name_method : ident) ,+) => { $ (impl $ AstIdNode for ast ::$ ident { }) + fn has_name_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap) -> Option < ErasedFileAstId > { match_ast ! { match node { $ (ast ::$ ident (node) => { let name = node .$ name_method () ; let name = name . as_ref () . map_or ("" , | it | it . text_non_mutable ()) ; let result = ErasedHasNameFileAstId { name , } ; Some (index_map . new_id (ErasedFileAstIdKind ::$ ident , result)) } ,) * _ => None , } } } fn should_alloc_has_name (kind : SyntaxKind) -> bool { false $ (|| ast ::$ ident :: can_cast (kind)) * } } ; }
    };
}

register_has_name_ast_id!();
macro_rules! deps {
    () => {
        ErasedAssocItemFileAstId!();
        AstIdNode!();
        ErasedFileAstId!();
        ErasedHasNameFileAstId!();
        ErasedFileAstIdKind!();
        ErasedAstIdNextIndexMap!();
    };
}

macro_rules! register_assoc_item_ast_id {
    () => {
        deps!();
        macro_rules ! register_assoc_item_ast_id { (impl $ AstIdNode : ident for $ ($ ident : ident = $ name_callback : expr) ,+) => { $ (impl $ AstIdNode for ast ::$ ident { }) + fn assoc_item_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap , parent : Option <& ErasedFileAstId >,) -> Option < ErasedFileAstId > { match_ast ! { match node { $ (ast ::$ ident (node) => { let name = $ name_callback (node) ; let name = name . as_ref () . map_or ("" , | it | it . text_non_mutable ()) ; let properties = ErasedHasNameFileAstId { name , } ; let result = ErasedAssocItemFileAstId { parent : parent . copied () , properties , } ; Some (index_map . new_id (ErasedFileAstIdKind ::$ ident , result)) } ,) * _ => None , } } } fn should_alloc_assoc_item (kind : SyntaxKind) -> bool { false $ (|| ast ::$ ident :: can_cast (kind)) * } } ; }
    };
}

register_assoc_item_ast_id!()
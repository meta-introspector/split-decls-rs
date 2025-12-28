macro_rules! deps {
    () => {
        ErasedAstIdNextIndexMap!();
        ImplFileAstId!();
        ErasedFileAstId!();
        ErasedFileAstIdKind!();
    };
}

macro_rules! impl_ast_id {
    () => {
        deps!();
        fn impl_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap ,) -> Option < ErasedFileAstId > { if let Some (node) = ast :: Impl :: cast (node . clone ()) { let type_as_name = | ty : Option < ast :: Type > | match ty ? { ast :: Type :: PathType (it) => Some (it . path () ? . segment () ? . name_ref () ?) , _ => None , } ; let self_ty_name = type_as_name (node . self_ty ()) ; let trait_name = type_as_name (node . trait_ ()) ; let data = ImplFileAstId { self_ty_name : self_ty_name . as_ref () . map (| it | it . text_non_mutable ()) , trait_name : trait_name . as_ref () . map (| it | it . text_non_mutable ()) , } ; Some (index_map . new_id (ErasedFileAstIdKind :: Impl , data)) } else { None } }
    };
}

impl_ast_id!();
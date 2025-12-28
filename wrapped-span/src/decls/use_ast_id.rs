macro_rules! deps {
    () => {
        ErasedAstIdNextIndexMap!();
        ErasedFileAstIdKind!();
        ErasedFileAstId!();
    };
}

macro_rules! use_ast_id {
    () => {
        deps!();
        fn use_ast_id (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap ,) -> Option < ErasedFileAstId > { if ast :: Use :: can_cast (node . kind ()) { Some (index_map . new_id (ErasedFileAstIdKind :: Use , ())) } else { None } }
    };
}

use_ast_id!()
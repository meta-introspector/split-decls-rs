macro_rules! deps {
    () => {
        ErasedFileAstIdKind!();
        ErasedFileAstId!();
    };
}

macro_rules! ROOT_ERASED_FILE_AST_ID {
    () => {
        deps!();
        # [doc = " The root ast id always points to the encompassing file, using this in spans is discouraged as"] # [doc = " any range relative to it will be effectively absolute, ruining the entire point of anchored"] # [doc = " relative text ranges."] pub const ROOT_ERASED_FILE_AST_ID : ErasedFileAstId = ErasedFileAstId (pack_hash_index_and_kind (0 , 0 , ErasedFileAstIdKind :: Root as u32)) ;
    };
}

ROOT_ERASED_FILE_AST_ID!();
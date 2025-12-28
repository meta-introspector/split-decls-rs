macro_rules! deps {
    () => {
        ErasedFileAstId!();
    };
}

macro_rules! hash_ast_id {
    () => {
        deps!();
        # [inline] fn hash_ast_id (ptr : & ErasedFileAstId) -> u64 { FxBuildHasher . hash_one (ptr) }
    };
}

hash_ast_id!();
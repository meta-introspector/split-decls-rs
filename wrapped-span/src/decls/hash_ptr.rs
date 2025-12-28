macro_rules! hash_ptr {
    () => {
        # [inline] fn hash_ptr (ptr : & SyntaxNodePtr) -> u64 { FxBuildHasher . hash_one (ptr) }
    };
}

hash_ptr!();
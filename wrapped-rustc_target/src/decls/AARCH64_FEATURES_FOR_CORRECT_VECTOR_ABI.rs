macro_rules! AARCH64_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const AARCH64_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "neon")] ;
    };
}

AARCH64_FEATURES_FOR_CORRECT_VECTOR_ABI!()
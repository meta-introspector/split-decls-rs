macro_rules! ARM_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const ARM_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "neon")] ;
    };
}

ARM_FEATURES_FOR_CORRECT_VECTOR_ABI!()
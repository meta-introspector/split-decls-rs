macro_rules! CSKY_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const CSKY_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "vdspv1")] ;
    };
}

CSKY_FEATURES_FOR_CORRECT_VECTOR_ABI!();
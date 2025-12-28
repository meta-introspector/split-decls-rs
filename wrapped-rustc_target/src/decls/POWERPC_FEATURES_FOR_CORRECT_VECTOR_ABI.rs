macro_rules! POWERPC_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const POWERPC_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "altivec")] ;
    };
}

POWERPC_FEATURES_FOR_CORRECT_VECTOR_ABI!();
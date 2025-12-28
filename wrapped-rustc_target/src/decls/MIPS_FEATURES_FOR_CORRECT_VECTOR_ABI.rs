macro_rules! MIPS_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const MIPS_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "msa")] ;
    };
}

MIPS_FEATURES_FOR_CORRECT_VECTOR_ABI!();
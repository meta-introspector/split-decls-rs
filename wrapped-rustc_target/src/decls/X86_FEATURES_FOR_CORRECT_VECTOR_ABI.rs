macro_rules! X86_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const X86_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "sse") , (256 , "avx") , (512 , "avx512f")] ;
    };
}

X86_FEATURES_FOR_CORRECT_VECTOR_ABI!();
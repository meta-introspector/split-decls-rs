macro_rules! S390X_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const S390X_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "vector")] ;
    };
}

S390X_FEATURES_FOR_CORRECT_VECTOR_ABI!()
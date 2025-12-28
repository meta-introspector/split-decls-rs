macro_rules! LOONGARCH_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const LOONGARCH_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "lsx") , (256 , "lasx")] ;
    };
}

LOONGARCH_FEATURES_FOR_CORRECT_VECTOR_ABI!()
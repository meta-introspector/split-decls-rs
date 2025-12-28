macro_rules! RISCV_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const RISCV_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(32 , "zvl32b") , (64 , "zvl64b") , (128 , "zvl128b") , (256 , "zvl256b") , (512 , "zvl512b") , (1024 , "zvl1024b") , (2048 , "zvl2048b") , (4096 , "zvl4096b") , (8192 , "zvl8192b") , (16384 , "zvl16384b") , (32768 , "zvl32768b") , (65536 , "zvl65536b") ,] ;
    };
}

RISCV_FEATURES_FOR_CORRECT_VECTOR_ABI!();
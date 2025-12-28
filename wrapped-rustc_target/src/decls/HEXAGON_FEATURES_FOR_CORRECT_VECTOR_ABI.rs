macro_rules! HEXAGON_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const HEXAGON_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(1024 , "hvx-length128b")] ;
    };
}

HEXAGON_FEATURES_FOR_CORRECT_VECTOR_ABI!();
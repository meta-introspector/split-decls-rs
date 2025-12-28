macro_rules! LITERALS_LENGTH_DEFAULT_DISTRIBUTION {
    () => {
        # [doc = " If [ModeType::Predefined] is selected for a symbol type, its FSE decoding"] # [doc = " table is generated using a predefined distribution table."] # [doc = ""] # [doc = " https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#literals-length"] const LITERALS_LENGTH_DEFAULT_DISTRIBUTION : [i32 ; 36] = [4 , 3 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 1 , 1 , 1 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 3 , 2 , 1 , 1 , 1 , 1 , 1 , - 1 , - 1 , - 1 , - 1 ,] ;
    };
}

LITERALS_LENGTH_DEFAULT_DISTRIBUTION!()
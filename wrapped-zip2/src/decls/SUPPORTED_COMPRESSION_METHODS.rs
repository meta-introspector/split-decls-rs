macro_rules! deps {
    () => {
        CompressionMethod!();
        Ppmd!();
    };
}

macro_rules! SUPPORTED_COMPRESSION_METHODS {
    () => {
        deps!();
        # [doc = " The compression methods which have been implemented."] pub const SUPPORTED_COMPRESSION_METHODS : & [CompressionMethod] = & [CompressionMethod :: Stored , # [cfg (feature = "_deflate-any")] CompressionMethod :: Deflated , # [cfg (feature = "deflate64")] CompressionMethod :: Deflate64 , # [cfg (feature = "bzip2")] CompressionMethod :: Bzip2 , # [cfg (feature = "zstd")] CompressionMethod :: Zstd , # [cfg (feature = "xz")] CompressionMethod :: Xz , # [cfg (feature = "ppmd")] CompressionMethod :: Ppmd ,] ;
    };
}

SUPPORTED_COMPRESSION_METHODS!()
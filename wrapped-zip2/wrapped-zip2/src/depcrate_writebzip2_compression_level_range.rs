// Generated macro for bzip2_compression_level_range (function)
macro_rules! Depcrate_writebzip2_compression_level_range {
() => {
// Module: crate::write
// Provides: {"bzip2_compression_level_range"}
// Dependencies: {}
# [cfg (feature = "bzip2")] fn bzip2_compression_level_range () -> std :: ops :: RangeInclusive < i64 > { let min = bzip2 :: Compression :: fast () . level () as i64 ; let max = bzip2 :: Compression :: best () . level () as i64 ; min ..= max }
};
}

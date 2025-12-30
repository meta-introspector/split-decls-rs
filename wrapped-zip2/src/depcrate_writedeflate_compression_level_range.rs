// Generated macro for deflate_compression_level_range (function)
macro_rules! Depcrate_writedeflate_compression_level_range {
() => {
// Module: crate::write
// Provides: {"deflate_compression_level_range"}
// Dependencies: {}
# [cfg (feature = "_deflate-any")] fn deflate_compression_level_range () -> std :: ops :: RangeInclusive < i64 > { # [cfg (feature = "deflate-flate2")] let min = Compression :: fast () . level () as i64 ; # [cfg (all (feature = "deflate-zopfli" , not (feature = "deflate-flate2")))] let min = 1 ; # [cfg (feature = "deflate-zopfli")] let max = 264 ; # [cfg (all (feature = "deflate-flate2" , not (feature = "deflate-zopfli")))] let max = Compression :: best () . level () as i64 ; min ..= max }
};
}

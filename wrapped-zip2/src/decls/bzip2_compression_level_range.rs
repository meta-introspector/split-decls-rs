macro_rules! bzip2_compression_level_range {
    () => {
        # [cfg (feature = "bzip2")] fn bzip2_compression_level_range () -> std :: ops :: RangeInclusive < i64 > { let min = bzip2 :: Compression :: fast () . level () as i64 ; let max = bzip2 :: Compression :: best () . level () as i64 ; min ..= max }
    };
}

bzip2_compression_level_range!()
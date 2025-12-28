macro_rules! deps {
    () => {
        ReduceDecoder!();
        ImplodeDecoder!();
        Lzma!();
        Ppmd!();
        ShrinkDecoder!();
    };
}

macro_rules! Decompressor {
    () => {
        deps!();
        pub (crate) enum Decompressor < R : io :: BufRead > { Stored (R) , # [cfg (feature = "deflate-flate2")] Deflated (flate2 :: bufread :: DeflateDecoder < R >) , # [cfg (feature = "deflate64")] Deflate64 (deflate64 :: Deflate64Decoder < R >) , # [cfg (feature = "bzip2")] Bzip2 (bzip2 :: bufread :: BzDecoder < R >) , # [cfg (feature = "zstd")] Zstd (zstd :: Decoder < 'static , R >) , # [cfg (feature = "lzma")] Lzma (Lzma < R >) , # [cfg (feature = "legacy-zip")] Shrink (crate :: legacy :: shrink :: ShrinkDecoder < R >) , # [cfg (feature = "legacy-zip")] Reduce (crate :: legacy :: reduce :: ReduceDecoder < R >) , # [cfg (feature = "legacy-zip")] Implode (crate :: legacy :: implode :: ImplodeDecoder < R >) , # [cfg (feature = "xz")] Xz (Box < lzma_rust2 :: XzReader < R > >) , # [cfg (feature = "ppmd")] Ppmd (Ppmd < R >) , }
    };
}

Decompressor!()
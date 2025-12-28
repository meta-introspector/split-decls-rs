macro_rules! deps {
    () => {
        Ppmd!();
        MaybeEncrypted!();
    };
}

macro_rules! GenericZipWriter {
    () => {
        deps!();
        enum GenericZipWriter < W : Write + Seek > { Closed , Storer (MaybeEncrypted < W >) , # [cfg (feature = "deflate-flate2")] Deflater (DeflateEncoder < MaybeEncrypted < W > >) , # [cfg (feature = "deflate-zopfli")] ZopfliDeflater (zopfli :: DeflateEncoder < MaybeEncrypted < W > >) , # [cfg (feature = "deflate-zopfli")] BufferedZopfliDeflater (BufWriter < zopfli :: DeflateEncoder < MaybeEncrypted < W > > >) , # [cfg (feature = "bzip2")] Bzip2 (BzEncoder < MaybeEncrypted < W > >) , # [cfg (feature = "zstd")] Zstd (ZstdEncoder < 'static , MaybeEncrypted < W > >) , # [cfg (feature = "xz")] Xz (Box < lzma_rust2 :: XzWriter < MaybeEncrypted < W > > >) , # [cfg (feature = "ppmd")] Ppmd (Box < ppmd_rust :: Ppmd8Encoder < MaybeEncrypted < W > > >) , }
    };
}

GenericZipWriter!()
macro_rules! Lzma {
    () => {
        # [cfg (feature = "lzma")] pub (crate) enum Lzma < R : io :: BufRead > { Uninitialized { reader : Option < R > , uncompressed_size : u64 , } , Initialized (Box < lzma_rust2 :: LzmaReader < R > >) , }
    };
}

Lzma!()
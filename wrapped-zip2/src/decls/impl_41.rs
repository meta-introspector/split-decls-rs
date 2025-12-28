macro_rules! deps {
    () => {
        ReduceDecoder!();
        ZipError!();
        Lzma!();
        Ppmd!();
        ZipResult!();
        ImplodeDecoder!();
        Decompressor!();
        ShrinkDecoder!();
        CompressionMethod!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < R : io :: BufRead > Decompressor < R > { pub fn new (reader : R , compression_method : CompressionMethod , # [cfg (any (feature = "lzma" , feature = "legacy-zip"))] uncompressed_size : u64 , # [cfg (not (any (feature = "lzma" , feature = "legacy-zip")))] _uncompressed_size : u64 , # [cfg (feature = "legacy-zip")] flags : u16 , # [cfg (not (feature = "legacy-zip"))] _flags : u16 ,) -> crate :: result :: ZipResult < Self > { Ok (match compression_method { CompressionMethod :: Stored => Decompressor :: Stored (reader) , # [cfg (feature = "deflate-flate2")] CompressionMethod :: Deflated => { Decompressor :: Deflated (flate2 :: bufread :: DeflateDecoder :: new (reader)) } # [cfg (feature = "deflate64")] CompressionMethod :: Deflate64 => { Decompressor :: Deflate64 (deflate64 :: Deflate64Decoder :: with_buffer (reader)) } # [cfg (feature = "bzip2")] CompressionMethod :: Bzip2 => Decompressor :: Bzip2 (bzip2 :: bufread :: BzDecoder :: new (reader)) , # [cfg (feature = "zstd")] CompressionMethod :: Zstd => Decompressor :: Zstd (zstd :: Decoder :: with_buffer (reader) ?) , # [cfg (feature = "lzma")] CompressionMethod :: Lzma => Decompressor :: Lzma (Lzma :: Uninitialized { reader : Some (reader) , uncompressed_size , }) , # [cfg (feature = "xz")] CompressionMethod :: Xz => { Decompressor :: Xz (Box :: new (lzma_rust2 :: XzReader :: new (reader , false))) } # [cfg (feature = "ppmd")] CompressionMethod :: Ppmd => Decompressor :: Ppmd (Ppmd :: Uninitialized (Some (reader))) , # [cfg (feature = "legacy-zip")] CompressionMethod :: Shrink => Decompressor :: Shrink (crate :: legacy :: shrink :: ShrinkDecoder :: new (reader , uncompressed_size) ,) , # [cfg (feature = "legacy-zip")] CompressionMethod :: Reduce (n) => Decompressor :: Reduce (crate :: legacy :: reduce :: ReduceDecoder :: new (reader , uncompressed_size , n) ,) , # [cfg (feature = "legacy-zip")] CompressionMethod :: Implode => Decompressor :: Implode (crate :: legacy :: implode :: ImplodeDecoder :: new (reader , uncompressed_size , flags) ,) , _ => { return Err (crate :: result :: ZipError :: UnsupportedArchive ("Compression method not supported" ,)) } }) } # [doc = " Consumes this decoder, returning the underlying reader."] # [allow (clippy :: infallible_destructuring_match)] pub fn into_inner (self) -> io :: Result < R > { let inner = match self { Decompressor :: Stored (r) => r , # [cfg (feature = "deflate-flate2")] Decompressor :: Deflated (r) => r . into_inner () , # [cfg (feature = "deflate64")] Decompressor :: Deflate64 (r) => r . into_inner () , # [cfg (feature = "bzip2")] Decompressor :: Bzip2 (r) => r . into_inner () , # [cfg (feature = "zstd")] Decompressor :: Zstd (r) => r . finish () , # [cfg (feature = "lzma")] Decompressor :: Lzma (r) => match r { Lzma :: Uninitialized { mut reader , .. } => reader . take () . ok_or_else (| | io :: Error :: other ("Reader was not set")) ? , Lzma :: Initialized (decoder) => decoder . into_inner () , } , # [cfg (feature = "legacy-zip")] Decompressor :: Shrink (r) => r . into_inner () , # [cfg (feature = "legacy-zip")] Decompressor :: Reduce (r) => r . into_inner () , # [cfg (feature = "legacy-zip")] Decompressor :: Implode (r) => r . into_inner () , # [cfg (feature = "xz")] Decompressor :: Xz (r) => r . into_inner () , # [cfg (feature = "ppmd")] Decompressor :: Ppmd (r) => match r { Ppmd :: Uninitialized (mut reader) => reader . take () . ok_or_else (| | io :: Error :: other ("Reader was not set")) ? , Ppmd :: Initialized (decoder) => decoder . into_inner () , } , } ; Ok (inner) } }
    };
}

impl_41!()
macro_rules! CompressionLevel {
    () => {
        # [doc = " The compression mode used impacts the speed of compression,"] # [doc = " and resulting compression ratios. Faster compression will result"] # [doc = " in worse compression ratios, and vice versa."] # [derive (Copy , Clone)] pub enum CompressionLevel { # [doc = " This level does not compress the data at all, and simply wraps"] # [doc = " it in a Zstandard frame."] Uncompressed , # [doc = " This level is roughly equivalent to Zstd compression level 1"] Fastest , # [doc = " This level is roughly equivalent to Zstd level 3,"] # [doc = " or the one used by the official compressor when no level"] # [doc = " is specified."] # [doc = ""] # [doc = " UNIMPLEMENTED"] Default , # [doc = " This level is roughly equivalent to Zstd level 7."] # [doc = ""] # [doc = " UNIMPLEMENTED"] Better , # [doc = " This level is roughly equivalent to Zstd level 11."] # [doc = ""] # [doc = " UNIMPLEMENTED"] Best , }
    };
}

CompressionLevel!()
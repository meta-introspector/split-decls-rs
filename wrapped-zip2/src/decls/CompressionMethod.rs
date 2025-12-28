macro_rules! deps {
    () => {
        ZipFileData!();
        Lzma!();
        Ppmd!();
    };
}

macro_rules! CompressionMethod {
    () => {
        deps!();
        # [allow (deprecated)] # [doc = " Identifies the storage format used to compress a file within a ZIP archive."] # [doc = ""] # [doc = " Each file's compression method is stored alongside it, allowing the"] # [doc = " contents to be read without context."] # [doc = ""] # [doc = " When creating ZIP files, you may choose the method to use with"] # [doc = " [`crate::write::FileOptions::compression_method`]"] # [derive (Copy , Clone , PartialEq , Eq , Debug)] # [cfg_attr (fuzzing , derive (arbitrary :: Arbitrary))] # [non_exhaustive] pub enum CompressionMethod { # [doc = " Store the file as is"] Stored , # [doc = " Compress the file using Deflate"] # [cfg (feature = "_deflate-any")] Deflated , # [doc = " Compress the file using Deflate64."] # [doc = " Decoding deflate64 is supported but encoding deflate64 is not supported."] # [cfg (feature = "deflate64")] Deflate64 , # [doc = " Compress the file using BZIP2"] # [cfg (feature = "bzip2")] Bzip2 , # [doc = " Encrypted using AES."] # [doc = ""] # [doc = " The actual compression method has to be taken from the AES extra data field"] # [doc = " or from `ZipFileData`."] # [cfg (feature = "aes-crypto")] Aes , # [doc = " Compress the file using ZStandard"] # [cfg (feature = "zstd")] Zstd , # [doc = " Compress the file using LZMA"] # [cfg (feature = "lzma")] Lzma , # [cfg (feature = "legacy-zip")] # [doc = " Method 1 Shrink"] Shrink , # [cfg (feature = "legacy-zip")] # [doc = " Reduce (Method 2-5)"] Reduce (u8) , # [cfg (feature = "legacy-zip")] # [doc = " Method 6 Implode/explode"] Implode , # [doc = " Compress the file using XZ"] # [cfg (feature = "xz")] Xz , # [doc = " Compress the file using PPMd"] # [cfg (feature = "ppmd")] Ppmd , # [doc = " Unsupported compression method"] # [cfg_attr (not (fuzzing) , deprecated (since = "0.5.7" , note = "use the constants instead"))] Unsupported (u16) , }
    };
}

CompressionMethod!();
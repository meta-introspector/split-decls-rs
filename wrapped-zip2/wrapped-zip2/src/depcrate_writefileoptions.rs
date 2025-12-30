// Generated macro for FileOptions (struct)
macro_rules! Depcrate_writeFileOptions {
() => {
// Module: crate::write
// Provides: {"FileOptions"}
// Dependencies: {}
# [doc = " Metadata for a file to be written"] # [derive (Clone , Debug , Copy , Eq , PartialEq)] pub struct FileOptions < 'k , T : FileOptionExtension > { pub (crate) compression_method : CompressionMethod , pub (crate) compression_level : Option < i64 > , pub (crate) last_modified_time : DateTime , pub (crate) permissions : Option < u32 > , pub (crate) large_file : bool , pub (crate) encrypt_with : Option < EncryptWith < 'k > > , pub (crate) extended_options : T , pub (crate) alignment : u16 , # [cfg (feature = "deflate-zopfli")] pub (super) zopfli_buffer_size : Option < usize > , # [cfg (feature = "aes-crypto")] pub (crate) aes_mode : Option < (AesMode , AesVendorVersion , CompressionMethod) > , }
};
}

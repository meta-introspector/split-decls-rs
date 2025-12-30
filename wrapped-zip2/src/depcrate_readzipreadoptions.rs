// Generated macro for ZipReadOptions (struct)
macro_rules! Depcrate_readZipReadOptions {
() => {
// Module: crate::read
// Provides: {"ZipReadOptions"}
// Dependencies: {}
# [doc = " Options for reading a file from an archive."] # [derive (Default)] pub struct ZipReadOptions < 'a > { # [doc = " The password to use when decrypting the file.  This is ignored if not required."] password : Option < & 'a [u8] > , # [doc = " Ignore the value of the encryption flag and proceed as if the file were plaintext."] ignore_encryption_flag : bool , }
};
}

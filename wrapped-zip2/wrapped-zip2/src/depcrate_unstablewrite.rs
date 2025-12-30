// Generated macro for write (module)
macro_rules! Depcrate_unstablewrite {
() => {
// Module: crate::unstable
// Provides: {"write"}
// Dependencies: {}
# [doc = " Types for creating ZIP archives."] pub mod write { use crate :: write :: { FileOptionExtension , FileOptions } ; # [doc = " Unstable methods for [`FileOptions`]."] pub trait FileOptionsExt { # [doc = " Write the file with the given password using the deprecated ZipCrypto algorithm."] # [doc = ""] # [doc = " This is not recommended for new archives, as ZipCrypto is not secure."] fn with_deprecated_encryption (self , password : & [u8]) -> Self ; } impl < T : FileOptionExtension > FileOptionsExt for FileOptions < '_ , T > { fn with_deprecated_encryption (self , password : & [u8]) -> FileOptions < 'static , T > { self . with_deprecated_encryption (password) } } }
};
}

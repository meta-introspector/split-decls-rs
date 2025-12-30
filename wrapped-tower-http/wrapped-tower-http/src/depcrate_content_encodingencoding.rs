// Generated macro for Encoding (enum)
macro_rules! Depcrate_content_encodingEncoding {
() => {
// Module: crate::content_encoding
// Provides: {"Encoding"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , Ord , PartialOrd , PartialEq , Eq)] pub (crate) enum Encoding { # [allow (dead_code)] Identity , # [cfg (any (feature = "fs" , feature = "compression-deflate"))] Deflate , # [cfg (any (feature = "fs" , feature = "compression-gzip"))] Gzip , # [cfg (any (feature = "fs" , feature = "compression-br"))] Brotli , # [cfg (any (feature = "fs" , feature = "compression-zstd"))] Zstd , }
};
}

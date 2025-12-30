// Generated macro for impl_409 (impl)
macro_rules! Depcrate_compression_utilsimpl_409 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_409"}
// Dependencies: {}
impl AcceptEncoding { # [allow (dead_code)] pub (crate) fn to_header_value (self) -> Option < HeaderValue > { let accept = match (self . gzip () , self . deflate () , self . br () , self . zstd ()) { (true , true , true , false) => "gzip,deflate,br" , (true , true , false , false) => "gzip,deflate" , (true , false , true , false) => "gzip,br" , (true , false , false , false) => "gzip" , (false , true , true , false) => "deflate,br" , (false , true , false , false) => "deflate" , (false , false , true , false) => "br" , (true , true , true , true) => "zstd,gzip,deflate,br" , (true , true , false , true) => "zstd,gzip,deflate" , (true , false , true , true) => "zstd,gzip,br" , (true , false , false , true) => "zstd,gzip" , (false , true , true , true) => "zstd,deflate,br" , (false , true , false , true) => "zstd,deflate" , (false , false , true , true) => "zstd,br" , (false , false , false , true) => "zstd" , (false , false , false , false) => return None , } ; Some (HeaderValue :: from_static (accept)) } # [allow (dead_code)] pub (crate) fn set_gzip (& mut self , enable : bool) { self . gzip = enable ; } # [allow (dead_code)] pub (crate) fn set_deflate (& mut self , enable : bool) { self . deflate = enable ; } # [allow (dead_code)] pub (crate) fn set_br (& mut self , enable : bool) { self . br = enable ; } # [allow (dead_code)] pub (crate) fn set_zstd (& mut self , enable : bool) { self . zstd = enable ; } }
};
}

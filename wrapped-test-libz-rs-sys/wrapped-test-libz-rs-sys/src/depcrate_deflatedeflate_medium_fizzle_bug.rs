// Generated macro for deflate_medium_fizzle_bug (function)
macro_rules! Depcrate_deflatedeflate_medium_fizzle_bug {
() => {
// Module: crate::deflate
// Provides: {"deflate_medium_fizzle_bug"}
// Dependencies: {}
# [test] fn deflate_medium_fizzle_bug () { const EXPECTED : & [u8] = & [120 , 156 , 99 , 96 , 128 , 3 , 73 , 6 , 26 , 3 , 71 , 218 , 2 , 28 , 182 , 214 , 17 , 225 , 50 , 85 , 100 , 30 , 0 , 132 , 7 , 24 , 220 ,] ; const INPUT : & str = "\0\0\0\0\0\0\0\0\0\0\0\u{19}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0%\0\0\0\0\0\0\0\0\0\0\0\0" ; let mut output = [0 ; EXPECTED . len ()] ; let config = DeflateConfig :: new (6) ; let (output , err) = zlib_rs :: deflate :: compress_slice (& mut output , INPUT . as_bytes () , config) ; assert_eq ! (err , ReturnCode :: Ok) ; assert_eq ! (output , EXPECTED) ; }
};
}

// Generated macro for hello_world_uncompressed (function)
macro_rules! Depcrate_inflatehello_world_uncompressed {
() => {
// Module: crate::inflate
// Provides: {"hello_world_uncompressed"}
// Dependencies: {}
# [test] fn hello_world_uncompressed () { let deflated = [0x78 , 0x01 , 0x01 , 0x0d , 0x00 , 0xf2 , 0xff , 0x48 , 0x65 , 0x6c , 0x6c , 0x6f , 0x20 , 0x57 , 0x6f , 0x72 , 0x6c , 0x64 , 0x21 , 0x0a , 0x20 , 0x91 , 0x04 , 0x48 ,] ; let output = uncompress_help (& deflated) ; assert_eq ! (String :: from_utf8 (output) . unwrap () , "Hello World!\n") ; }
};
}

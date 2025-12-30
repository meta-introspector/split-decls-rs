// Generated macro for hello_world_fixed (function)
macro_rules! Depcrate_inflatehello_world_fixed {
() => {
// Module: crate::inflate
// Provides: {"hello_world_fixed"}
// Dependencies: {}
# [test] fn hello_world_fixed () { let deflated = [0x78 , 0x01 , 0xf3 , 0x48 , 0xcd , 0xc9 , 0xc9 , 0x57 , 0x08 , 0xcf , 0x2f , 0xca , 0x49 , 0x51 , 0xe4 , 0x02 , 0x00 , 0x20 , 0x91 , 0x04 , 0x48 ,] ; let output = uncompress_help (& deflated) ; assert_eq ! (String :: from_utf8 (output) . unwrap () , "Hello World!\n") ; }
};
}

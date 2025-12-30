// Generated macro for hello_world_dynamic (function)
macro_rules! Depcrate_inflatehello_world_dynamic {
() => {
// Module: crate::inflate
// Provides: {"hello_world_dynamic"}
// Dependencies: {}
# [test] fn hello_world_dynamic () { let input = "\n\0\0\0\0l\0\nl\0l\0l\u{1}llll\n" ; let deflated = [120 , 156 , 5 , 193 , 177 , 1 , 0 , 0 , 8 , 195 , 160 , 184 , 246 , 86 , 254 , 159 , 133 , 85 , 105 , 146 , 131 , 61 , 24 , 141 , 3 , 128 ,] ; let output = uncompress_help (& deflated) ; assert_eq ! (String :: from_utf8 (output) . unwrap () , input) ; }
};
}

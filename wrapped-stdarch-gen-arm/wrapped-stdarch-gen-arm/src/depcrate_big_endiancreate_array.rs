// Generated macro for create_array (function)
macro_rules! Depcrate_big_endiancreate_array {
() => {
// Module: crate::big_endian
// Provides: {"create_array"}
// Dependencies: {}
# [doc = " To compose the simd_shuffle! call we need:"] # [doc = " - simd_shuffle!(<arg1>, <arg2>, <array>)"] # [doc = ""] # [doc = " Here we are creating a string version of the `<array>` that can be used as an"] # [doc = " Expression Identifier"] # [doc = ""] # [doc = " In textual form `a: int32x4_t` which has 4 lanes would generate:"] # [doc = " ```"] # [doc = " [0, 1, 2, 3]"] # [doc = " ```"] fn create_array (lanes : u32) -> Option < String > { match lanes { 1 => None , 2 => Some ("[1, 0]" . to_string ()) , 3 => Some ("[2, 1, 0]" . to_string ()) , 4 => Some ("[3, 2, 1, 0]" . to_string ()) , 8 => Some ("[7, 6, 5, 4, 3, 2, 1, 0]" . to_string ()) , 16 => Some ("[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]" . to_string ()) , _ => panic ! ("Incorrect vector number of vector lanes: {lanes}") , } }
};
}

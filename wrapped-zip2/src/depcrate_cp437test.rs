// Generated macro for test (module)
macro_rules! Depcrate_cp437test {
() => {
// Module: crate::cp437
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn to_char_valid () { for i in 0x00_u32 .. 0x100 { super :: to_char (i as u8) ; } } # [test] fn ascii () { for i in 0x00 .. 0x80 { assert_eq ! (super :: to_char (i) , i as char) ; } } # [test] # [allow (unknown_lints)] # [allow (invalid_from_utf8)] fn example_slice () { use super :: FromCp437 ; let data = b"Cura\x87ao" ; assert ! (:: std :: str :: from_utf8 (data) . is_err ()) ; assert_eq ! (data . from_cp437 () , "Curaçao") ; } # [test] fn example_vec () { use super :: FromCp437 ; let data = vec ! [0xCC , 0xCD , 0xCD , 0xB9] ; assert ! (String :: from_utf8 (data . clone ()) . is_err ()) ; assert_eq ! (&* data . from_cp437 () , "╠══╣") ; } }
};
}

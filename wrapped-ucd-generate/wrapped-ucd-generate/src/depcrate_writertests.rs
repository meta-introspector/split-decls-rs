// Generated macro for tests (module)
macro_rules! Depcrate_writertests {
() => {
// Module: crate::writer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: WriterBuilder ; use super :: { pack_str , rust_type_name } ; use crate :: error :: Error ; use std :: io :: Cursor ; fn unpack_str (mut encoded : u64) -> String { let mut value = String :: new () ; while encoded != 0 { value . push ((encoded & 0xFF) as u8 as char) ; encoded = encoded >> 8 ; } value } # [test] fn packed () { assert_eq ! ("G" , unpack_str (pack_str ("G") . unwrap ())) ; assert_eq ! ("GG" , unpack_str (pack_str ("GG") . unwrap ())) ; assert_eq ! ("YEO" , unpack_str (pack_str ("YEO") . unwrap ())) ; assert_eq ! ("ABCDEFGH" , unpack_str (pack_str ("ABCDEFGH") . unwrap ())) ; assert_eq ! ("" , unpack_str (pack_str ("") . unwrap ())) ; assert ! (pack_str ("ABCDEFGHI") . is_err ()) ; assert ! (pack_str ("AB\x00CD") . is_err ()) ; } # [test] fn test_rust_type_name () { assert_eq ! (& rust_type_name ("simple") , "Simple") ; assert_eq ! (& rust_type_name ("SCRIPT") , "SCRIPT") ; assert_eq ! (& rust_type_name ("dot.separated") , "DotSeparated") ; assert_eq ! (& rust_type_name ("dash-separated") , "DashSeparated") ; assert_eq ! (& rust_type_name ("white \tspace") , "WhiteSpace") ; assert_eq ! (& rust_type_name ("snake_case") , "SnakeCase") ; } # [test] fn codepoint_to_codepoint_fn_error () { let cursor = Cursor :: new (Vec :: new ()) ; let builder = WriterBuilder :: new ("test") ; let mut writer = builder . from_writer (cursor) ; let map = [(1 , 0)] . iter () . copied () . collect () ; match writer . codepoint_to_codepoint_fn ("err" , & map) { Err (Error :: Other (msg)) => { assert ! (msg . contains ("destination codepoint must not be 0")) } res => panic ! ("expected error matching, \
                 'destination codepoint must not be 0' \
                 got: {:?}" , res) , } } }
};
}

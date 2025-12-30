// Generated macro for decode_tests (module)
macro_rules! Depcrate_responsedecode_tests {
() => {
// Module: crate::response
// Provides: {"decode_tests"}
// Dependencies: {}
# [cfg (test)] mod decode_tests { use super :: decode_body ; # [test] fn utf8 () { let input = "Rød grød med fløde" ; assert_eq ! (decode_body (input . as_bytes () . to_vec () , Some ("utf-8")) . unwrap () , input , "Parses utf-8") ; } # [test] fn default_utf8 () { let input = "Rød grød med fløde" ; assert_eq ! (decode_body (input . as_bytes () . to_vec () , None) . unwrap () , input , "Defaults to utf-8") ; } # [test] fn euc_kr () { let input = vec ! [0xb3 , 0xbb , 0x20 , 0xc7 , 0xb0 , 0xc0 , 0xb8 , 0xb7 , 0xce , 0x20 , 0xb5 , 0xb9 , 0xbe , 0xc6 , 0xbf , 0xc0 , 0xb6 , 0xf3 , 0x2c , 0x20 , 0xb3 , 0xbb , 0x20 , 0xbe , 0xc8 , 0xbf , 0xa1 , 0xbc , 0xad , 0x20 , 0xc0 , 0xe1 , 0xb5 , 0xe9 , 0xb0 , 0xc5 , 0xb6 , 0xf3 ,] ; let result = decode_body (input , Some ("euc-kr")) ; if cfg ! (feature = "encoding") { assert_eq ! (result . unwrap () , "내 품으로 돌아오라, 내 안에서 잠들거라") ; } else { assert ! (result . is_err () , "Only utf-8 is supported") ; } } }
};
}

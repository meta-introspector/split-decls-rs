// Generated macro for test_normalize_newlines (function)
macro_rules! Depcrate_symbol_teststest_normalize_newlines {
() => {
// Module: crate::symbol::tests
// Provides: {"test_normalize_newlines"}
// Dependencies: {}
# [test] fn test_normalize_newlines () { fn check (before : & str , after : & str , expected_positions : & [u32]) { let mut actual = before . to_string () ; let mut actual_positions = vec ! [] ; normalize_newlines (& mut actual , & mut actual_positions) ; let actual_positions : Vec < _ > = actual_positions . into_iter () . map (| nc | nc . pos . 0) . collect () ; assert_eq ! (actual . as_str () , after) ; assert_eq ! (actual_positions , expected_positions) ; } check ("" , "" , & []) ; check ("\n" , "\n" , & []) ; check ("\r" , "\r" , & []) ; check ("\r\r" , "\r\r" , & []) ; check ("\r\n" , "\n" , & [1]) ; check ("hello world" , "hello world" , & []) ; check ("hello\nworld" , "hello\nworld" , & []) ; check ("hello\r\nworld" , "hello\nworld" , & [6]) ; check ("\r\nhello\r\nworld\r\n" , "\nhello\nworld\n" , & [1 , 7 , 13]) ; check ("\r\r\n" , "\r\n" , & [2]) ; check ("hello\rworld" , "hello\rworld" , & []) ; }
};
}

// Generated macro for test (module)
macro_rules! Depcrate_utilstest {
() => {
// Module: crate::utils
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_remove_trailing_white_spaces () { let s = "    r#\"\n        test\n    \"#" ; assert_eq ! (remove_trailing_white_spaces (s) , s) ; } # [test] fn test_trim_left_preserve_layout () { let s = "aaa\n\tbbb\n    ccc" ; let config = Config :: default () ; let indent = Indent :: new (4 , 0) ; assert_eq ! (trim_left_preserve_layout (s , indent , & config) , Some ("aaa\n    bbb\n    ccc" . to_string ())) ; } }
};
}

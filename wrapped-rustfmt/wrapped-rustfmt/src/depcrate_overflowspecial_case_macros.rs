// Generated macro for SPECIAL_CASE_MACROS (const)
macro_rules! Depcrate_overflowSPECIAL_CASE_MACROS {
() => {
// Module: crate::overflow
// Provides: {"SPECIAL_CASE_MACROS"}
// Dependencies: {}
# [doc = " A list of `format!`-like macros, that take a long format string and a list of arguments to"] # [doc = " format."] # [doc = ""] # [doc = " Organized as a list of `(&str, usize)` tuples, giving the name of the macro and the number of"] # [doc = " arguments before the format string (none for `format!(\"format\", ...)`, one for `assert!(result,"] # [doc = " \"format\", ...)`, two for `assert_eq!(left, right, \"format\", ...)`)."] const SPECIAL_CASE_MACROS : & [(& str , usize)] = & [("eprint!" , 0) , ("eprintln!" , 0) , ("format!" , 0) , ("format_args!" , 0) , ("print!" , 0) , ("println!" , 0) , ("panic!" , 0) , ("unreachable!" , 0) , ("debug!" , 0) , ("error!" , 0) , ("info!" , 0) , ("warn!" , 0) , ("assert!" , 1) , ("debug_assert!" , 1) , ("write!" , 1) , ("writeln!" , 1) , ("assert_eq!" , 2) , ("assert_ne!" , 2) , ("debug_assert_eq!" , 2) , ("debug_assert_ne!" , 2) ,] ;
};
}

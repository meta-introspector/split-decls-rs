use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Test a single wrapped module in isolation"] # [doc = " "] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " test_wrapper!(\"path/to/generated/wrapper\", original_tests);"] # [doc = " ```"] # [proc_macro] pub fn test_wrapper (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as TestWrapperInput) ; let wrapper_path = & input . wrapper_path ; let test_name = & input . test_name ; quote ! { # [cfg (test)] mod # test_name { use super ::*; use # wrapper_path ; # [test] fn test_wrapper_compiles () { assert ! (true) ; } # [test] fn test_wrapper_interface () { } } } . into () }
}
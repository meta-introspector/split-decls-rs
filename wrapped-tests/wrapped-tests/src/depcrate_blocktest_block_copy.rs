// Generated macro for test_block_copy (function)
macro_rules! Depcrate_blocktest_block_copy {
() => {
// Module: crate::block
// Provides: {"test_block_copy"}
// Dependencies: {}
# [test] fn test_block_copy () { let s = "Hello!" . to_string () ; let expected_len = s . len () as i32 ; let closure = move | | s . len () as i32 ; for block in [StackBlock :: new (closure . clone ()) , StackBlock :: with_encoding :: < VoidToInt > (closure) ,] { assert_eq ! (unsafe { invoke_int_block (& block) } , expected_len) ; let copied = block . copy () ; assert_eq ! (unsafe { invoke_int_block (& copied) } , expected_len) ; } }
};
}

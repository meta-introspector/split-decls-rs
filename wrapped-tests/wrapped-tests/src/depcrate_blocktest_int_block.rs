// Generated macro for test_int_block (function)
macro_rules! Depcrate_blocktest_int_block {
() => {
// Module: crate::block
// Provides: {"test_int_block"}
// Dependencies: {}
# [test] fn test_int_block () { # [track_caller] fn invoke_assert (block : & Block < dyn Fn () -> i32 > , expected : i32) { assert_eq ! (block . call (()) , expected) ; assert_eq ! (unsafe { invoke_int_block (block) } , expected) ; } global_block ! { static GLOBAL_BLOCK = || -> i32 { 42 } ; } invoke_assert (unsafe { & * get_int_block () } , 7) ; invoke_assert (& unsafe { RcBlock :: from_raw (get_int_block_with (3)) } . unwrap () , 3 ,) ; invoke_assert (& StackBlock :: new (| | 10) , 10) ; invoke_assert (& RcBlock :: new (| | 6) , 6) ; invoke_assert (& StackBlock :: with_encoding :: < VoidToInt > (| | 10) , 10) ; invoke_assert (& RcBlock :: with_encoding :: < _ , _ , _ , VoidToInt > (| | 6) , 6) ; invoke_assert (& GLOBAL_BLOCK , 42) ; }
};
}

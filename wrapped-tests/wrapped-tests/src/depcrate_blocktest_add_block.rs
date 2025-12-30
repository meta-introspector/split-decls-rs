// Generated macro for test_add_block (function)
macro_rules! Depcrate_blocktest_add_block {
() => {
// Module: crate::block
// Provides: {"test_add_block"}
// Dependencies: {}
# [test] fn test_add_block () { # [track_caller] fn invoke_assert (block : & Block < dyn Fn (i32) -> i32 > , expected : i32) { assert_eq ! (block . call ((5 ,)) , expected) ; assert_eq ! (unsafe { invoke_add_block (block , 5) } , expected) ; } global_block ! { static GLOBAL_BLOCK = | x : i32 | -> i32 { x + 42 } ; } invoke_assert (unsafe { & * get_add_block () } , 12) ; invoke_assert (& unsafe { RcBlock :: from_raw (get_add_block_with (3)) } . unwrap () , 8 ,) ; invoke_assert (& StackBlock :: new (| a : i32 | a + 6) , 11) ; invoke_assert (& RcBlock :: new (| a : i32 | a + 6) , 11) ; invoke_assert (& StackBlock :: with_encoding :: < IntToInt > (| a : i32 | a + 6) , 11) ; invoke_assert (& RcBlock :: with_encoding :: < _ , _ , _ , IntToInt > (| a : i32 | a + 6) , 11 ,) ; invoke_assert (& GLOBAL_BLOCK , 47) ; }
};
}

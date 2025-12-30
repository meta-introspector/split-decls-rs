// Generated macro for test_block_stack_move (function)
macro_rules! Depcrate_blocktest_block_stack_move {
() => {
// Module: crate::block
// Provides: {"test_block_stack_move"}
// Dependencies: {}
# [test] fn test_block_stack_move () { fn make_block () -> StackBlock < 'static , () , i32 , impl Fn () -> i32 > { let x = 7 ; StackBlock :: new (move | | x) } fn make_block_with_encoding () -> StackBlock < 'static , () , i32 , impl Fn () -> i32 > { let x = 7 ; StackBlock :: with_encoding :: < VoidToInt > (move | | x) } for block in [& make_block () as & Block < dyn Fn () -> i32 > , & make_block_with_encoding () as & Block < dyn Fn () -> i32 > ,] { assert_eq ! (unsafe { invoke_int_block (block) } , 7) ; } }
};
}

// Generated macro for seq_struct_borrow (function)
macro_rules! Depcrate_macros_testsseq_struct_borrow {
() => {
// Module: crate::macros::tests
// Provides: {"seq_struct_borrow"}
// Dependencies: {}
# [test] fn seq_struct_borrow () { # ! [allow (dead_code)] # [derive (Debug , PartialEq)] struct Point { x : u32 , y : u32 , } fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , Point > { let mut dec_uint = digit0 . parse_to () ; seq ! { Point { x : dec_uint , _ : ',' , y : dec_uint , _ : empty } } . parse_next (input) } }
};
}

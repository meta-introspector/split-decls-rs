// Generated macro for seq_tuple_borrow (function)
macro_rules! Depcrate_macros_testsseq_tuple_borrow {
() => {
// Module: crate::macros::tests
// Provides: {"seq_tuple_borrow"}
// Dependencies: {}
# [test] fn seq_tuple_borrow () { # ! [allow (dead_code)] # [derive (Debug , PartialEq)] struct Point (u32 , u32) ; fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , Point > { let mut dec_uint = digit0 . parse_to () ; seq ! { Point (dec_uint , _ : ',' , dec_uint , _ : empty) } . parse_next (input) } }
};
}

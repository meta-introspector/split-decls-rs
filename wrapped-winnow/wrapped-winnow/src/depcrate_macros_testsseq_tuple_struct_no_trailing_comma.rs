// Generated macro for seq_tuple_struct_no_trailing_comma (function)
macro_rules! Depcrate_macros_testsseq_tuple_struct_no_trailing_comma {
() => {
// Module: crate::macros::tests
// Provides: {"seq_tuple_struct_no_trailing_comma"}
// Dependencies: {}
# [test] fn seq_tuple_struct_no_trailing_comma () { # ! [allow (dead_code)] # [derive (Debug , PartialEq)] struct Point (u32 , u32) ; fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , Point > { seq ! { Point (dec_uint , _ : ',' , dec_uint) } . parse_next (input) } }
};
}

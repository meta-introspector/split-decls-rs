// Generated macro for seq_struct_trailing_comma_elided (function)
macro_rules! Depcrate_macros_testsseq_struct_trailing_comma_elided {
() => {
// Module: crate::macros::tests
// Provides: {"seq_struct_trailing_comma_elided"}
// Dependencies: {}
# [test] fn seq_struct_trailing_comma_elided () { # ! [allow (dead_code)] # [derive (Debug , PartialEq)] struct Point { x : u32 , y : u32 , } fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , Point > { seq ! { Point { x : dec_uint , _ : ',' , y : dec_uint , _ : empty , } } . parse_next (input) } }
};
}

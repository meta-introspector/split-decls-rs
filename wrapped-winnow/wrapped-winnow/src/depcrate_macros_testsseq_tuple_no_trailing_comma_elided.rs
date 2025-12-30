// Generated macro for seq_tuple_no_trailing_comma_elided (function)
macro_rules! Depcrate_macros_testsseq_tuple_no_trailing_comma_elided {
() => {
// Module: crate::macros::tests
// Provides: {"seq_tuple_no_trailing_comma_elided"}
// Dependencies: {}
# [test] fn seq_tuple_no_trailing_comma_elided () { # ! [allow (dead_code)] fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , (u32 , u32) > { seq ! { (dec_uint , _ : ',' , dec_uint , _ : empty) } . parse_next (input) } }
};
}

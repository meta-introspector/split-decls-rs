// Generated macro for seq_tuple_no_parens (function)
macro_rules! Depcrate_macros_testsseq_tuple_no_parens {
() => {
// Module: crate::macros::tests
// Provides: {"seq_tuple_no_parens"}
// Dependencies: {}
# [test] fn seq_tuple_no_parens () { # ! [allow (dead_code)] fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , (u32 , u32) > { seq ! (dec_uint , _ : ',' , dec_uint ,) . parse_next (input) } }
};
}

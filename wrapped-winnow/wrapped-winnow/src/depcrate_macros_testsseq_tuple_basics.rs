// Generated macro for seq_tuple_basics (function)
macro_rules! Depcrate_macros_testsseq_tuple_basics {
() => {
// Module: crate::macros::tests
// Provides: {"seq_tuple_basics"}
// Dependencies: {}
# [test] fn seq_tuple_basics () { fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , (u32 , u32) > { seq ! { (dec_uint , _ : ',' , dec_uint ,) } . parse_next (input) } assert_parse ! (parser . parse_peek ("123,4 remaining") , str ! [[r#"
Ok(
    (
        " remaining",
        (
            123,
            4,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (parser . parse_peek ("123, remaining") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: " remaining",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (parser . parse_peek ("") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "",
        },
    ),
)

"#]] . raw ()) ; }
};
}

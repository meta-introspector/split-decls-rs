// Generated macro for seq_tuple_struct_basics (function)
macro_rules! Depcrate_macros_testsseq_tuple_struct_basics {
() => {
// Module: crate::macros::tests
// Provides: {"seq_tuple_struct_basics"}
// Dependencies: {}
# [test] fn seq_tuple_struct_basics () { # [derive (Debug , PartialEq)] struct Point (u32 , u32) ; fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , Point > { seq ! { Point (dec_uint , _ : ',' , dec_uint ,) } . parse_next (input) } assert_parse ! (parser . parse_peek ("123,4 remaining") , str ! [[r#"
Ok(
    (
        " remaining",
        Point(
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

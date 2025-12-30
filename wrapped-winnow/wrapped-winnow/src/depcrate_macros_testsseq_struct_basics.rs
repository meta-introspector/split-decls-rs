// Generated macro for seq_struct_basics (function)
macro_rules! Depcrate_macros_testsseq_struct_basics {
() => {
// Module: crate::macros::tests
// Provides: {"seq_struct_basics"}
// Dependencies: {}
# [test] fn seq_struct_basics () { # [derive (Debug , PartialEq)] struct Point { x : u32 , y : u32 , } fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , Point > { seq ! { Point { x : dec_uint , _ : ',' , y : dec_uint , } } . parse_next (input) } assert_parse ! (parser . parse_peek ("123,4 remaining") , str ! [[r#"
Ok(
    (
        " remaining",
        Point {
            x: 123,
            y: 4,
        },
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

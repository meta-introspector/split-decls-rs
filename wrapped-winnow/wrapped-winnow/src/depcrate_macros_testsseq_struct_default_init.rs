// Generated macro for seq_struct_default_init (function)
macro_rules! Depcrate_macros_testsseq_struct_default_init {
() => {
// Module: crate::macros::tests
// Provides: {"seq_struct_default_init"}
// Dependencies: {}
# [test] fn seq_struct_default_init () { # [derive (Debug , PartialEq , Default)] struct Point { x : u32 , y : u32 , z : u32 , } fn parser < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , Point > { seq ! { Point { x : dec_uint , _ : ',' , y : dec_uint , .. Default :: default () } } . parse_next (input) } assert_parse ! (parser . parse_peek ("123,4 remaining") , str ! [[r#"
Ok(
    (
        " remaining",
        Point {
            x: 123,
            y: 4,
            z: 0,
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

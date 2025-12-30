// Generated macro for dispatch_basics (function)
macro_rules! Depcrate_macros_testsdispatch_basics {
() => {
// Module: crate::macros::tests
// Provides: {"dispatch_basics"}
// Dependencies: {}
# [test] fn dispatch_basics () { fn escape_seq_char < 'i > (input : & mut & 'i str) -> TestResult < & 'i str , char > { dispatch ! { any ; 'b' => empty . value ('\u{8}') , 'f' => empty . value ('\u{c}') , 'n' => empty . value ('\n') , 'r' => empty . value ('\r') , 't' => empty . value ('\t') , '\\' => empty . value ('\\') , '"' => empty . value ('"') , _ => fail ::< _ , char , _ >, } . parse_next (input) } assert_parse ! (escape_seq_char . parse_peek ("b123") , str ! [[r#"
Ok(
    (
        "123",
        '\u{8}',
    ),
)

"#]] . raw ()) ; assert_parse ! (escape_seq_char . parse_peek ("error") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "rror",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (escape_seq_char . parse_peek ("") , str ! [[r#"
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

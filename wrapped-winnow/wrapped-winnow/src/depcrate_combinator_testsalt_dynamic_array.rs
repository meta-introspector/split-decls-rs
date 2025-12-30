// Generated macro for alt_dynamic_array (function)
macro_rules! Depcrate_combinator_testsalt_dynamic_array {
() => {
// Module: crate::combinator::tests
// Provides: {"alt_dynamic_array"}
// Dependencies: {}
# [test] fn alt_dynamic_array () { fn alt1 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { alt (& mut ["a" , "bc" , "def"] [..]) . parse_next (i) } let a = & b"a" [..] ; assert_parse ! (alt1 . parse_peek (a) , str ! [[r#"
Ok(
    (
        [],
        [
            97,
        ],
    ),
)

"#]] . raw ()) ; let bc = & b"bc" [..] ; assert_parse ! (alt1 . parse_peek (bc) , str ! [[r#"
Ok(
    (
        [],
        [
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; let defg = & b"defg" [..] ; assert_parse ! (alt1 . parse_peek (defg) , str ! [[r#"
Ok(
    (
        [
            103,
        ],
        [
            100,
            101,
            102,
        ],
    ),
)

"#]] . raw ()) ; }
};
}

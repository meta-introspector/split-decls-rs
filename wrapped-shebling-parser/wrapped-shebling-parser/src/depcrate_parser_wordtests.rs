// Generated macro for tests (module)
macro_rules! Depcrate_parser_wordtests {
() => {
// Module: crate::parser::word
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: parser :: tests ; use insta :: assert_debug_snapshot ; # [test] fn test_identifier () { assert_debug_snapshot ! (tests :: parse_ok (identifier , "foo" , "") , @ r###"
        (
            [],
            "foo",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (identifier , "_foo0" , "") , @ r###"
        (
            [],
            "_foo0",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_fail (identifier , "0foo") , @ r###"
        (
            ParseError {
                location: 0,
                notes: [],
            },
            [],
        )
        "###) ; } }
};
}

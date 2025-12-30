// Generated macro for tests (module)
macro_rules! Depcrate_parser_quotedtests {
() => {
// Module: crate::parser::quoted
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use insta :: assert_debug_snapshot ; use super :: * ; use crate :: parser :: tests ; # [test] fn test_single_quoted () { assert_debug_snapshot ! (tests :: parse_ok (single_quoted , "'foo bar'" , "") , @ r###"
        (
            [],
            "foo bar",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (single_quoted , "'let's" , "s") , @ r###"
        (
            [
                (
                    "shebling::bad_quote",
                    SourceSpan {
                        offset: SourceOffset(
                            4,
                        ),
                        length: 1,
                    },
                ),
            ],
            "let",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (single_quoted , "'let\\'s" , "s") , @ r###"
        (
            [
                (
                    "shebling::bad_escape",
                    SourceSpan {
                        offset: SourceOffset(
                            5,
                        ),
                        length: 0,
                    },
                ),
            ],
            "let\\",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_fail (single_quoted , "'foo") , @ r###"
        (
            ParseError {
                location: 4,
                notes: [
                    ParseErrorNote {
                        location: 4,
                        note: "expected ending single quote!",
                    },
                ],
            },
            [],
        )
        "###) ; } # [test] fn test_double_quoted () { } }
};
}

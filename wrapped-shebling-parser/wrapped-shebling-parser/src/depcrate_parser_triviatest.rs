// Generated macro for test (module)
macro_rules! Depcrate_parser_triviatest {
() => {
// Module: crate::parser::trivia
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use insta :: assert_debug_snapshot ; use super :: * ; use crate :: parser :: tests ; # [test] fn test_carriage_return () { assert_debug_snapshot ! (tests :: parse_ok (carriage_return , "\r" , "") , @ r###"
        (
            [
                (
                    "shebling::carriage_return",
                    SourceSpan {
                        offset: SourceOffset(
                            0,
                        ),
                        length: 1,
                    },
                ),
            ],
            '\r',
        )
        "###) ; } # [test] fn test_line_ending () { assert_debug_snapshot ! (tests :: parse_ok (line_ending , "\n" , "") , @ r###"
        (
            [],
            '\n',
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (line_ending , "\r\n" , "") , @ r###"
        (
            [
                (
                    "shebling::carriage_return",
                    SourceSpan {
                        offset: SourceOffset(
                            0,
                        ),
                        length: 1,
                    },
                ),
            ],
            '\n',
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_fail (line_ending , "\r") , @ r###"
        (
            ParseError {
                location: 1,
                notes: [],
            },
            [
                (
                    "shebling::carriage_return",
                    SourceSpan {
                        offset: SourceOffset(
                            0,
                        ),
                        length: 1,
                    },
                ),
            ],
        )
        "###) ; } # [test] fn test_line_space () { assert_debug_snapshot ! (tests :: parse_ok (line_space , " " , "") , @ r###"
        (
            [],
            ' ',
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (line_space , "\t" , "") , @ r###"
        (
            [],
            '\t',
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (line_space , "\u{A0}" , "") , @ r###"
        (
            [
                (
                    "shebling::unichar",
                    SourceSpan {
                        offset: SourceOffset(
                            0,
                        ),
                        length: 2,
                    },
                ),
            ],
            ' ',
        )
        "###) ; } # [test] fn test_trivia () { assert_debug_snapshot ! (tests :: parse_ok (trivia , "" , "") , @ r###"
        (
            [],
            "",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (trivia , "foo" , "foo") , @ r###"
        (
            [],
            "",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (trivia , " \t " , "") , @ r###"
        (
            [],
            " \t ",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (trivia , "# foo" , "") , @ r###"
        (
            [],
            "# foo",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (trivia , "#foo \\\n" , "\n") , @ r###"
        (
            [],
            "#foo \\",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (trivia , " \\\n#foo" , "") , @ r###"
        (
            [],
            " #foo",
        )
        "###) ; assert_debug_snapshot ! (tests :: parse_ok (trivia , " \\\n#foo \\\n" , "\n") , @ r###"
        (
            [
                (
                    "shebling::bad_escape",
                    SourceSpan {
                        offset: SourceOffset(
                            9,
                        ),
                        length: 0,
                    },
                ),
            ],
            " #foo \\",
        )
        "###) ; } }
};
}

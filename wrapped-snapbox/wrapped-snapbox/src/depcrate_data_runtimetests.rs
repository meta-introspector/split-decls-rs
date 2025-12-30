// Generated macro for tests (module)
macro_rules! Depcrate_data_runtimetests {
() => {
// Module: crate::data::runtime
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: assert_data_eq ; use crate :: prelude :: * ; use crate :: str ; # [test] fn test_format_patch () { let patch = format_patch ("hello\nworld\n") ; assert_data_eq ! (patch , str ! [[r##"
[r#"
hello
world

"#]
"##]] ,) ; let patch = format_patch (r"hello\tworld") ; assert_data_eq ! (patch , str ! [[r##"[r#"hello\tworld"#]"##]] . raw ()) ; let patch = format_patch ("{\"foo\": 42}") ; assert_data_eq ! (patch , str ! [[r##"[r#"{"foo": 42}"#]"##]]) ; } # [test] fn test_patchwork () { let mut patchwork = Patchwork :: new ("one two three" . to_owned ()) ; patchwork . patch (4 .. 7 , "zwei") . unwrap () ; patchwork . patch (0 .. 3 , "один") . unwrap () ; patchwork . patch (8 .. 13 , "3") . unwrap () ; assert_data_eq ! (patchwork . to_debug () , str ! [[r#"
Patchwork {
    text: "один zwei 3",
    indels: {
        OrdRange {
            start: 0,
            end: 3,
        }: (
            8,
            "один",
        ),
        OrdRange {
            start: 4,
            end: 7,
        }: (
            4,
            "zwei",
        ),
        OrdRange {
            start: 8,
            end: 13,
        }: (
            1,
            "3",
        ),
    },
}

"#]] ,) ; } # [test] fn test_patchwork_overlap_diverge () { let mut patchwork = Patchwork :: new ("one two three" . to_owned ()) ; patchwork . patch (4 .. 7 , "zwei") . unwrap () ; patchwork . patch (4 .. 7 , "abcd") . unwrap_err () ; assert_data_eq ! (patchwork . to_debug () , str ! [[r#"
Patchwork {
    text: "one zwei three",
    indels: {
        OrdRange {
            start: 4,
            end: 7,
        }: (
            4,
            "zwei",
        ),
    },
}

"#]] ,) ; } # [test] fn test_patchwork_overlap_converge () { let mut patchwork = Patchwork :: new ("one two three" . to_owned ()) ; patchwork . patch (4 .. 7 , "zwei") . unwrap () ; patchwork . patch (4 .. 7 , "zwei") . unwrap () ; assert_data_eq ! (patchwork . to_debug () , str ! [[r#"
Patchwork {
    text: "one zwei three",
    indels: {
        OrdRange {
            start: 4,
            end: 7,
        }: (
            4,
            "zwei",
        ),
    },
}

"#]] ,) ; } # [test] fn test_locate () { macro_rules ! check_locate { ($ ([[$ s : literal]]) ,* $ (,) ?) => { $ ({ let lit = stringify ! ($ s) ; let with_trailer = format ! ("{} \t]]\n" , lit) ; assert_eq ! (locate_end (& with_trailer) , Some (lit . len ())) ; }) * } ; } check_locate ! ([[r#"{ arr: [[1, 2], [3, 4]], other: "foo" } "#]] , [["]]"]] , [["\"]]"]] , [[r#""]]"#]] ,) ; assert_eq ! (locate_end ("]]") , Some (0)) ; } # [test] fn test_find_str_lit_len () { macro_rules ! check_str_lit_len { ($ ($ s : literal) ,* $ (,) ?) => { $ ({ let lit = stringify ! ($ s) ; assert_eq ! (find_str_lit_len (lit) , Some (lit . len ())) ; }) * } } check_str_lit_len ! [r##"foa\""#"## , r##"

                asdf][]]""""#
            "## , "" , "\"" , "\"\"" , "#\"#\"#" ,] ; } }
};
}

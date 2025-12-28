macro_rules! deps {
    () => {
        StrLitKind!();
    };
}

macro_rules! format_patch {
    () => {
        deps!();
        fn format_patch (patch : & str) -> String { let lit_kind = lit_kind_for_patch (patch) ; let is_multiline = patch . contains ('\n') ; let mut buf = String :: new () ; if matches ! (lit_kind , StrLitKind :: Raw (_)) { buf . push ('[') ; } lit_kind . write_start (& mut buf) . unwrap () ; if is_multiline { buf . push ('\n') ; } buf . push_str (patch) ; if is_multiline { buf . push ('\n') ; } lit_kind . write_end (& mut buf) . unwrap () ; if matches ! (lit_kind , StrLitKind :: Raw (_)) { buf . push (']') ; } buf }
    };
}

format_patch!();
// Generated macro for format_patch (function)
macro_rules! Depcrate_data_runtimeformat_patch {
() => {
// Module: crate::data::runtime
// Provides: {"format_patch"}
// Dependencies: {}
fn format_patch (patch : & str) -> String { let lit_kind = lit_kind_for_patch (patch) ; let is_multiline = patch . contains ('\n') ; let mut buf = String :: new () ; if matches ! (lit_kind , StrLitKind :: Raw (_)) { buf . push ('[') ; } lit_kind . write_start (& mut buf) . unwrap () ; if is_multiline { buf . push ('\n') ; } buf . push_str (patch) ; if is_multiline { buf . push ('\n') ; } lit_kind . write_end (& mut buf) . unwrap () ; if matches ! (lit_kind , StrLitKind :: Raw (_)) { buf . push (']') ; } buf }
};
}

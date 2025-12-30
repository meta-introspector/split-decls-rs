// Generated macro for lit_kind_for_patch (function)
macro_rules! Depcrate_data_runtimelit_kind_for_patch {
() => {
// Module: crate::data::runtime
// Provides: {"lit_kind_for_patch"}
// Dependencies: {}
fn lit_kind_for_patch (patch : & str) -> StrLitKind { let has_dquote = patch . chars () . any (| c | c == '"') ; if ! has_dquote { let has_bslash_or_newline = patch . chars () . any (| c | matches ! (c , '\\' | '\n')) ; return if has_bslash_or_newline { StrLitKind :: Raw (1) } else { StrLitKind :: Normal } ; } let leading_hashes = | s : & str | s . chars () . take_while (| & c | c == '#') . count () ; let max_hashes = patch . split ('"') . map (leading_hashes) . max () . unwrap () ; StrLitKind :: Raw (max_hashes + 1) }
};
}

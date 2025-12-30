// Generated macro for newline_for_brace (function)
macro_rules! Depcrate_itemsnewline_for_brace {
() => {
// Module: crate::items
// Provides: {"newline_for_brace"}
// Dependencies: {}
fn newline_for_brace (config : & Config , where_clause : & ast :: WhereClause) -> FnBraceStyle { let predicate_count = where_clause . predicates . len () ; if config . where_single_line () && predicate_count == 1 { return FnBraceStyle :: SameLine ; } let brace_style = config . brace_style () ; let use_next_line = brace_style == BraceStyle :: AlwaysNextLine || (brace_style == BraceStyle :: SameLineWhere && predicate_count > 0) ; if use_next_line { FnBraceStyle :: NextLine } else { FnBraceStyle :: SameLine } }
};
}

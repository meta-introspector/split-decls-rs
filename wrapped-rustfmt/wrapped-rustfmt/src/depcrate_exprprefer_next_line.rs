// Generated macro for prefer_next_line (function)
macro_rules! Depcrate_exprprefer_next_line {
() => {
// Module: crate::expr
// Provides: {"prefer_next_line"}
// Dependencies: {}
# [doc = " Returns true if formatting next_line_rhs is better on a new line when compared to the"] # [doc = " original's line formatting."] # [doc = ""] # [doc = " It is considered better if:"] # [doc = " 1. the tactic is ForceNextLineWithoutIndent"] # [doc = " 2. next_line_rhs doesn't have newlines"] # [doc = " 3. the original line has more newlines than next_line_rhs"] # [doc = " 4. the original formatting of the first line ends with `(`, `{`, or `[` and next_line_rhs"] # [doc = "    doesn't"] pub (crate) fn prefer_next_line (orig_rhs : & str , next_line_rhs : & str , rhs_tactics : RhsTactics ,) -> bool { rhs_tactics == RhsTactics :: ForceNextLineWithoutIndent || ! next_line_rhs . contains ('\n') || count_newlines (orig_rhs) > count_newlines (next_line_rhs) + 1 || first_line_ends_with (orig_rhs , '(') && ! first_line_ends_with (next_line_rhs , '(') || first_line_ends_with (orig_rhs , '{') && ! first_line_ends_with (next_line_rhs , '{') || first_line_ends_with (orig_rhs , '[') && ! first_line_ends_with (next_line_rhs , '[') }
};
}

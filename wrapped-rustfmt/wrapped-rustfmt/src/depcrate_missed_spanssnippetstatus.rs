// Generated macro for SnippetStatus (struct)
macro_rules! Depcrate_missed_spansSnippetStatus {
() => {
// Module: crate::missed_spans
// Provides: {"SnippetStatus"}
// Dependencies: {}
struct SnippetStatus { # [doc = " An offset to the current line from the beginning of the original snippet."] line_start : usize , # [doc = " A length of trailing whitespaces on the current line."] last_wspace : Option < usize > , # [doc = " The current line number."] cur_line : usize , }
};
}

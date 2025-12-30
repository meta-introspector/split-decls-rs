// Generated macro for indent_next_line (function)
macro_rules! Depcrate_utilsindent_next_line {
() => {
// Module: crate::utils
// Provides: {"indent_next_line"}
// Dependencies: {}
# [doc = " Based on the given line, determine if the next line can be indented or not."] # [doc = " This allows to preserve the indentation of multi-line literals when"] # [doc = " re-inserted a code block that has been formatted separately from the rest"] # [doc = " of the code, such as code in macro defs or code blocks doc comments."] pub (crate) fn indent_next_line (kind : FullCodeCharKind , line : & str , config : & Config) -> bool { if kind . is_string () { config . format_strings () && line . ends_with ('\\') } else if config . style_edition () >= StyleEdition :: Edition2024 { ! kind . is_commented_string () } else { true } }
};
}

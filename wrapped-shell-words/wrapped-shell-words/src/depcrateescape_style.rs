// Generated macro for escape_style (function)
macro_rules! Depcrateescape_style {
() => {
// Module: crate
// Provides: {"escape_style"}
// Dependencies: {}
# [doc = " Determines escaping style to use."] fn escape_style (s : & str) -> EscapeStyle { if s . is_empty () { return EscapeStyle :: SingleQuoted ; } let mut special = false ; let mut newline = false ; let mut single_quote = false ; for c in s . chars () { match c { '\n' => { newline = true ; special = true ; } '\'' => { single_quote = true ; special = true ; } '|' | '&' | ';' | '<' | '>' | '(' | ')' | '$' | '`' | '\\' | '"' | ' ' | '\t' | '*' | '?' | '[' | '#' | '~' | '=' | '%' => { special = true ; } _ => continue , } } if ! special { EscapeStyle :: None } else if newline && ! single_quote { EscapeStyle :: SingleQuoted } else { EscapeStyle :: Mixed } }
};
}

// Generated macro for parsing (module)
macro_rules! Depcrate_identparsing {
() => {
// Module: crate::ident
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub mod parsing { use super :: * ; use synom :: { Synom , PResult , Cursor , parse_error } ; impl Synom for Ident { fn parse (input : Cursor) -> PResult < Self > { let (rest , span , sym) = match input . word () { Some (word) => word , _ => return parse_error () , } ; if sym . as_str () . starts_with ('\'') { return parse_error () ; } match sym . as_str () { "abstract" | "alignof" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "offsetof" | "override" | "priv" | "proc" | "pub" | "pure" | "ref" | "return" | "Self" | "self" | "sizeof" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" => return parse_error () , _ => { } } Ok ((rest , Ident { span : Span (span) , sym : sym , })) } fn description () -> Option < & 'static str > { Some ("identifier") } } }
};
}

// Generated macro for parsing (module)
macro_rules! Depcrate_identparsing {
() => {
// Module: crate::ident
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] mod parsing { use crate :: buffer :: Cursor ; use crate :: error :: Result ; use crate :: parse :: { Parse , ParseStream } ; use crate :: token :: Token ; use proc_macro2 :: Ident ; fn accept_as_ident (ident : & Ident) -> bool { match ident . to_string () . as_str () { "_" | "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "Self" | "self" | "static" | "struct" | "super" | "trait" | "true" | "try" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" => false , _ => true , } } # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Ident { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | { if let Some ((ident , rest)) = cursor . ident () { if accept_as_ident (& ident) { Ok ((ident , rest)) } else { Err (cursor . error (format_args ! ("expected identifier, found keyword `{}`" , ident ,))) } } else { Err (cursor . error ("expected identifier")) } }) } } impl Token for Ident { fn peek (cursor : Cursor) -> bool { if let Some ((ident , _rest)) = cursor . ident () { accept_as_ident (& ident) } else { false } } fn display () -> & 'static str { "identifier" } } }
};
}

// Generated macro for rewrite_macro_with_items (function)
macro_rules! Depcrate_macrosrewrite_macro_with_items {
() => {
// Module: crate::macros
// Provides: {"rewrite_macro_with_items"}
// Dependencies: {}
fn rewrite_macro_with_items (context : & RewriteContext < '_ > , items : & [MacroArg] , macro_name : & str , shape : Shape , style : Delimiter , original_style : Delimiter , position : MacroPosition , span : Span ,) -> RewriteResult { let style_to_delims = | style | match style { Delimiter :: Parenthesis => Ok (("(" , ")")) , Delimiter :: Bracket => Ok (("[" , "]")) , Delimiter :: Brace => Ok ((" {" , "}")) , _ => Err (RewriteError :: Unknown) , } ; let (opener , closer) = style_to_delims (style) ? ; let (original_opener , _) = style_to_delims (original_style) ? ; let trailing_semicolon = match style { Delimiter :: Parenthesis | Delimiter :: Bracket if position == MacroPosition :: Item => ";" , _ => "" , } ; let mut visitor = FmtVisitor :: from_context (context) ; visitor . block_indent = shape . indent . block_indent (context . config) ; visitor . last_pos = context . snippet_provider . span_after (span , original_opener . trim ()) ; for item in items { let item = match item { MacroArg :: Item (item) => item , _ => return Err (RewriteError :: Unknown) , } ; visitor . visit_item (item) ; } let mut result = String :: with_capacity (256) ; result . push_str (macro_name) ; result . push_str (opener) ; result . push_str (& visitor . block_indent . to_string_with_newline (context . config)) ; result . push_str (visitor . buffer . trim ()) ; result . push_str (& shape . indent . to_string_with_newline (context . config)) ; result . push_str (closer) ; result . push_str (trailing_semicolon) ; Ok (result) }
};
}

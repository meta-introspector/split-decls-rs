// Generated macro for shell_lex (function)
macro_rules! Depcrateshell_lex {
() => {
// Module: crate
// Provides: {"shell_lex"}
// Dependencies: {}
fn shell_lex (cmd : & str , call_site : Span ,) -> impl Iterator < Item = Result < (bool , bool , TokenStream) > > + '_ { tokenize (cmd) . map (move | token | { let token = token ? ; let mut splat = false ; let ts = match token . kind { TokenKind :: Word => parse_ts (& format ! ("(\"{}\")" , token . text)) , TokenKind :: String => parse_ts (& format ! ("(\"{}\")" , trim_decorations (token . text))) , TokenKind :: Interpolation { splat : s } => { splat = s ; let text = trim_decorations (token . text) ; let text = & text [.. text . len () - (if splat { "..." . len () } else { 0 })] ; if ! (text . chars () . all (| c | c . is_ascii_alphanumeric () || c == '_')) { return Err (format ! ("can only interpolate simple variables, got this expression instead: `{}`" , text)) ; } let ts = if splat { format ! ("({})" , text) } else { format ! ("(&({}))" , text) } ; respan (parse_ts (& ts) , call_site) } } ; Ok ((token . joined_to_prev , splat , ts)) }) }
};
}

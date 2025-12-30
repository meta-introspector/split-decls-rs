// Generated macro for parse_ignore (function)
macro_rules! Depcrateparse_ignore {
() => {
// Module: crate
// Provides: {"parse_ignore"}
// Dependencies: {}
fn parse_ignore (body : & mut std :: iter :: Peekable < token_stream :: IntoIter > , token : & TokenTree ,) -> Result < Option < (Option < Literal > , Span) > , proc_macro :: TokenStream > { match token { TokenTree :: Punct (op) if op . as_char () == '#' => () , _ => return Ok (None) , } let group = match body . peek () { Some (TokenTree :: Group (group)) if group . delimiter () == Delimiter :: Bracket => group , _ => return Ok (None) , } ; let mut stream = group . stream () . into_iter () ; let mut span = match stream . next () { Some (TokenTree :: Ident (token)) if token == "ignore" => token . span () , _ => return Ok (None) , } ; let ignore = span ; match stream . next () { Some (TokenTree :: Punct (op)) if op . as_char () == '=' => () , Some (token) => { return Err (compile_error (token . span () , "malformed `#[ignore = \"...\"]` attribute" ,)) } None => { return Ok (Some ((None , ignore))) ; } } if let Some (TokenTree :: Literal (lit)) = stream . next () { span = lit . span () ; let string = lit . to_string () ; if string . starts_with ('"') && string . ends_with ('"') { return Ok (Some ((Some (lit) , ignore))) ; } } Err (compile_error (span , "malformed `#[ignore]` attribute")) }
};
}

// Generated macro for delim_token_to_str (function)
macro_rules! Depcrate_macrosdelim_token_to_str {
() => {
// Module: crate::macros
// Provides: {"delim_token_to_str"}
// Dependencies: {}
fn delim_token_to_str (context : & RewriteContext < '_ > , delim_token : Delimiter , shape : Shape , use_multiple_lines : bool , inner_is_empty : bool ,) -> (String , String) { let (lhs , rhs) = match delim_token { Delimiter :: Parenthesis => ("(" , ")") , Delimiter :: Bracket => ("[" , "]") , Delimiter :: Brace => { if inner_is_empty || use_multiple_lines { ("{" , "}") } else { ("{ " , " }") } } Delimiter :: Invisible (_) => unreachable ! () , } ; if use_multiple_lines { let indent_str = shape . indent . to_string_with_newline (context . config) ; let nested_indent_str = shape . indent . block_indent (context . config) . to_string_with_newline (context . config) ; (format ! ("{lhs}{nested_indent_str}") , format ! ("{indent_str}{rhs}") ,) } else { (lhs . to_owned () , rhs . to_owned ()) } }
};
}

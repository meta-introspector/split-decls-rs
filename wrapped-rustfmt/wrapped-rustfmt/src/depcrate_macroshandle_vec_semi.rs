// Generated macro for handle_vec_semi (function)
macro_rules! Depcrate_macroshandle_vec_semi {
() => {
// Module: crate::macros
// Provides: {"handle_vec_semi"}
// Dependencies: {}
fn handle_vec_semi (context : & RewriteContext < '_ > , shape : Shape , arg_vec : Vec < MacroArg > , macro_name : String , delim_token : Delimiter , span : Span ,) -> RewriteResult { let (left , right) = match delim_token { Delimiter :: Parenthesis => ("(" , ")") , Delimiter :: Bracket => ("[" , "]") , _ => unreachable ! () , } ; let mac_shape = shape . offset_left (macro_name . len () , span) ? ; let total_overhead = 8 ; let nested_shape = mac_shape . block_indent (context . config . tab_spaces ()) ; let lhs = arg_vec [0] . rewrite_result (context , nested_shape) ? ; let rhs = arg_vec [1] . rewrite_result (context , nested_shape) ? ; if ! lhs . contains ('\n') && ! rhs . contains ('\n') && lhs . len () + rhs . len () + total_overhead <= shape . width { Ok (format ! ("{macro_name}{left}{lhs}; {rhs}{right}")) } else { Ok (format ! ("{}{}{}{};{}{}{}{}" , macro_name , left , nested_shape . indent . to_string_with_newline (context . config) , lhs , nested_shape . indent . to_string_with_newline (context . config) , rhs , shape . indent . to_string_with_newline (context . config) , right)) } }
};
}

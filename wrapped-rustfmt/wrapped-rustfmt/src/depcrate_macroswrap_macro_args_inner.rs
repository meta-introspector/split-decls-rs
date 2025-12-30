// Generated macro for wrap_macro_args_inner (function)
macro_rules! Depcrate_macroswrap_macro_args_inner {
() => {
// Module: crate::macros
// Provides: {"wrap_macro_args_inner"}
// Dependencies: {}
fn wrap_macro_args_inner (context : & RewriteContext < '_ > , args : & [ParsedMacroArg] , shape : Shape , use_multiple_lines : bool ,) -> RewriteResult { let mut result = String :: with_capacity (128) ; let mut iter = args . iter () . peekable () ; let indent_str = shape . indent . to_string_with_newline (context . config) ; while let Some (arg) = iter . next () { result . push_str (& arg . rewrite (context , shape , use_multiple_lines) ?) ; if use_multiple_lines && (arg . kind . ends_with_space () || iter . peek () . map_or (false , | a | a . kind . has_meta_var ())) { if arg . kind . ends_with_space () { result . pop () ; } result . push_str (& indent_str) ; } else if let Some (next_arg) = iter . peek () { let space_before_dollar = ! arg . kind . ends_with_space () && next_arg . kind . starts_with_dollar () ; let space_before_brace = next_arg . kind . starts_with_brace () ; if space_before_dollar || space_before_brace { result . push (' ') ; } } } if ! use_multiple_lines && result . len () >= shape . width { Err (RewriteError :: Unknown) } else { Ok (result) } }
};
}

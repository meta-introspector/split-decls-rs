// Generated macro for try_cmd (function)
macro_rules! Depcratetry_cmd {
() => {
// Module: crate
// Provides: {"try_cmd"}
// Dependencies: {}
fn try_cmd (macro_arg : TokenStream) -> Result < TokenStream > { let (cmd , literal) = { let mut iter = macro_arg . into_iter () ; let cmd = iter . next () . unwrap () ; let literal = iter . next () . unwrap () ; assert ! (iter . next () . is_none ()) ; (cmd , literal) } ; let literal = match into_literal (& literal) { Some (it) => it , None => return Err ("expected a plain string literal" . to_string ()) , } ; let literal_text = literal . to_string () ; if ! literal_text . starts_with ('"') { return Err ("expected a plain string literal" . to_string ()) ; } let mut args = shell_lex (literal_text . as_str () , literal . span ()) ; let mut res = TokenStream :: new () ; { let (_joined_to_prev , splat , program) = args . next () . ok_or_else (| | "command can't be empty" . to_string ()) ? ? ; if splat { return Err ("can't splat program name" . to_string ()) ; } res . extend (Some (cmd)) ; res . extend (program) ; } let mut prev_spat = false ; for arg in args { let (joined_to_prev , splat , arg) = arg ? ; if prev_spat && joined_to_prev { return Err (format ! ("can't combine splat with concatenation, add spaces around `{{{}...}}`" , trim_decorations (& res . into_iter () . last () . unwrap () . to_string ()) ,)) ; } prev_spat = splat ; let method = match (joined_to_prev , splat) { (false , false) => ".arg" , (false , true) => ".args" , (true , false) => ".__extend_arg" , (true , true) => { return Err (format ! ("can't combine splat with concatenation, add spaces around `{{{}...}}`" , trim_decorations (& arg . to_string ()) ,)) } } ; res . extend (parse_ts (method)) ; res . extend (arg) ; } Ok (res) }
};
}

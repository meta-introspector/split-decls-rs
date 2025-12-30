// Generated macro for parse_macro_fn (function)
macro_rules! Depcrate_parserparse_macro_fn {
() => {
// Module: crate::parser
// Provides: {"parse_macro_fn"}
// Dependencies: {}
fn parse_macro_fn (pair : Pair < Rule >) -> TeraResult < (String , HashMap < String , Option < Expr > >) > { let mut name = String :: new () ; let mut args = HashMap :: new () ; for p2 in pair . into_inner () { match p2 . as_rule () { Rule :: ident => name = p2 . as_str () . to_string () , Rule :: macro_def_arg => { let mut arg_name = None ; let mut default_val = None ; for p3 in p2 . into_inner () { match p3 . as_rule () { Rule :: ident => arg_name = Some (p3 . as_str () . to_string ()) , _ => default_val = Some (Expr :: new (parse_macro_arg (p3) ?)) , } ; } args . insert (arg_name . unwrap () , default_val) ; } _ => continue , } } Ok ((name , args)) }
};
}

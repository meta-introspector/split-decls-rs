// Generated macro for debug_print_generated (function)
macro_rules! Depcratedebug_print_generated {
() => {
// Module: crate
// Provides: {"debug_print_generated"}
// Dependencies: {}
fn debug_print_generated (ast : & DeriveInput , toks : & TokenStream) { let debug = env :: var ("STRUM_DEBUG") ; if let Ok (s) = debug { if s == "1" { println ! ("{}" , toks) ; } if ast . ident == s { println ! ("{}" , toks) ; } } }
};
}

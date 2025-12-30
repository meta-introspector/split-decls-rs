// Generated macro for assert_next_token (macro)
macro_rules! Depcrate_serassert_next_token {
() => {
// Module: crate::ser
// Provides: {"assert_next_token"}
// Dependencies: {}
macro_rules ! assert_next_token { ($ ser : expr , $ actual : ident) => { { assert_next_token ! ($ ser , stringify ! ($ actual) , Token ::$ actual , true) ; } } ; ($ ser : expr , $ actual : ident ($ v : expr)) => { { assert_next_token ! ($ ser , format_args ! (concat ! (stringify ! ($ actual) , "({:?})") , $ v) , Token ::$ actual (v) , v == $ v) ; } } ; ($ ser : expr , $ actual : ident { $ ($ k : ident) ,* }) => { { let compare = ($ ($ k ,) *) ; let field_format = || { use std :: fmt :: Write ; let mut buffer = String :: new () ; $ (write ! (& mut buffer , concat ! (stringify ! ($ k) , ": {:?}, ") , $ k) . unwrap () ;) * buffer } ; assert_next_token ! ($ ser , format_args ! (concat ! (stringify ! ($ actual) , " {{ {}}}") , field_format ()) , Token ::$ actual { $ ($ k) ,* } , ($ ($ k ,) *) == compare) ; } } ; ($ ser : expr , $ actual : expr , $ pat : pat , $ guard : expr) => { match $ ser . next_token () { Some ($ pat) if $ guard => { } Some (expected) => return Err (ser :: Error :: custom (format ! ("expected Token::{} but serialized as {}" , expected , $ actual))) , None => return Err (ser :: Error :: custom (format ! ("expected end of tokens, but {} was serialized" , $ actual))) , } } ; }
};
}

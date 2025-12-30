// Generated macro for ensure_ranged (macro)
macro_rules! Depcrate_internal_macrosensure_ranged {
() => {
// Module: crate::internal_macros
// Provides: {"ensure_ranged"}
// Dependencies: {}
# [doc = " Constructs a ranged integer, returning a `ComponentRange` error if the value is out of range."] macro_rules ! ensure_ranged { ($ type : ty : $ value : ident) => { match <$ type >:: new ($ value) { Some (val) => val , None => { $ crate :: hint :: cold_path () ; # [allow (trivial_numeric_casts)] return Err (crate :: error :: ComponentRange { name : stringify ! ($ value) , minimum : <$ type >:: MIN . get () as i64 , maximum : <$ type >:: MAX . get () as i64 , value : $ value as i64 , conditional_message : None , }) ; } } } ; ($ type : ty : $ value : ident $ (as $ as_type : ident) ? * $ factor : expr) => { match ($ value $ (as $ as_type) ?) . checked_mul ($ factor) { Some (val) => match <$ type >:: new (val) { Some (val) => val , None => { $ crate :: hint :: cold_path () ; # [allow (trivial_numeric_casts)] return Err (crate :: error :: ComponentRange { name : stringify ! ($ value) , minimum : <$ type >:: MIN . get () as i64 / $ factor as i64 , maximum : <$ type >:: MAX . get () as i64 / $ factor as i64 , value : $ value as i64 , conditional_message : None , }) ; } } , None => { $ crate :: hint :: cold_path () ; return Err (crate :: error :: ComponentRange { name : stringify ! ($ value) , minimum : <$ type >:: MIN . get () as i64 / $ factor as i64 , maximum : <$ type >:: MAX . get () as i64 / $ factor as i64 , value : $ value as i64 , conditional_message : None , }) ; } } } ; }
};
}

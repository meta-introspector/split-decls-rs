// Generated macro for assert_parse_eq (macro)
macro_rules! Depcrate_parseassert_parse_eq {
() => {
// Module: crate::parse
// Provides: {"assert_parse_eq"}
// Dependencies: {}
# [cfg (test)] # [macro_export] # [doc = " Helper macro to test the equality of the actual and expected parse."] macro_rules ! assert_parse_eq { ($ parser : expr , bytes $ src : expr , $ expected : expr $ (,) ?) => { $ crate :: assert_parse_ok ! ($ parser , bytes $ src) ; assert_eq ! ($ parser . parse ($ src) . unwrap () . 0 , $ expected , "expected {:?}, parse as {:?} but got {:?}" , $ src , $ expected , $ parser . easy_parse ($ src) . unwrap () . 0 ,) } ; ($ parser : expr , $ src : expr , $ expected : expr $ (,) ?) => { $ crate :: assert_parse_ok ! ($ parser , $ src) ; assert_eq ! ($ parser . parse ($ src . as_bytes ()) . unwrap () . 0 , $ expected , "expected {:?}, parse as {:?} but got {:?}" , $ src , $ expected , $ parser . easy_parse ($ src . as_bytes ()) . unwrap () . 0 ,) } ; }
};
}

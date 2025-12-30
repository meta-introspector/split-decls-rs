// Generated macro for assert_parse_err (macro)
macro_rules! Depcrate_parseassert_parse_err {
() => {
// Module: crate::parse
// Provides: {"assert_parse_err"}
// Dependencies: {}
# [cfg (test)] # [macro_export] # [doc = " Helper macro to test that a parse should fail with Err()."] macro_rules ! assert_parse_err { ($ parser : expr , bytes $ src : expr $ (,) ?) => { assert ! ($ parser . parse ($ src) . is_err () , "expected {:?}, parse {:?} as Err(), but got Ok() {:#?}" , stringify ! ($ parser) , $ src , $ parser . easy_parse ($ src) . unwrap () . 0 ,) } ; ($ parser : expr , $ src : expr $ (,) ?) => { assert ! ($ parser . parse ($ src . as_bytes ()) . is_err () , "expected {}, parse {} as Err(), but got Ok() {:#?}" , stringify ! ($ parser) , $ src , $ parser . easy_parse ($ src . as_bytes ()) . unwrap () . 0 ,) } ; }
};
}

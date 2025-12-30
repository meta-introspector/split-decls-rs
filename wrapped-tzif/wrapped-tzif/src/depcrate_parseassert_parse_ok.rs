// Generated macro for assert_parse_ok (macro)
macro_rules! Depcrate_parseassert_parse_ok {
() => {
// Module: crate::parse
// Provides: {"assert_parse_ok"}
// Dependencies: {}
# [cfg (test)] # [macro_export] # [doc = " Helper macro to test that a parse should succeed with Ok()."] macro_rules ! assert_parse_ok { ($ parser : expr , bytes $ src : expr $ (,) ?) => { assert ! ($ parser . parse ($ src) . is_ok () , "expected {:?}, parse {:?} as Ok(), but got Err() {:#?}" , stringify ! ($ parser) , $ src , $ parser . easy_parse ($ src) ,) } ; ($ parser : expr , $ src : expr $ (,) ?) => { assert ! ($ parser . parse (($ src) . as_bytes ()) . is_ok () , "expected {}, parse {} as Ok(), but got Err() {:#?}" , stringify ! ($ parser) , $ src , $ parser . easy_parse ($ src . as_bytes ()) ,) } ; }
};
}

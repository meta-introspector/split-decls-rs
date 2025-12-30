// Generated macro for assert_lex_rule (macro)
macro_rules! Depcrate_parser_tests_lexerassert_lex_rule {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"assert_lex_rule"}
// Dependencies: {}
macro_rules ! assert_lex_rule { ($ rule : expr , $ input : expr) => { let res = TeraParser :: parse ($ rule , $ input) ; println ! ("{:?}" , $ input) ; println ! ("{:#?}" , res) ; if res . is_err () { println ! ("{}" , res . unwrap_err ()) ; panic ! () ; } assert ! (res . is_ok ()) ; assert_eq ! (res . unwrap () . last () . unwrap () . as_span () . end () , $ input . len ()) ; } ; }
};
}

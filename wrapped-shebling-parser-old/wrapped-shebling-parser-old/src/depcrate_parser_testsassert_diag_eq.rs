// Generated macro for assert_diag_eq (macro)
macro_rules! Depcrate_parser_testsassert_diag_eq {
() => {
// Module: crate::parser::tests
// Provides: {"assert_diag_eq"}
// Dependencies: {}
macro_rules ! assert_diag_eq { ($ diag : expr , (($ line : literal , $ col : literal) , $ kind : path)) => { assert_diag_eq ! ($ diag , (($ line , $ col) , ($ line , $ col) , $ kind)) } ; ($ diag : expr , (($ line1 : literal , $ col1 : literal) , ($ line2 : literal , $ col2 : literal) , $ kind : path)) => { let actual_code = :: miette :: Diagnostic :: code ($ diag) . expect ("Diagnostic should have a code.") . to_string () ; let expected_code = :: miette :: Diagnostic :: code (&$ kind) . expect ("Diagnostic should have a code.") . to_string () ; :: pretty_assertions :: assert_str_eq ! (actual_code , expected_code) ; let (start , end) = ($ diag . range () . start () , $ diag . range () . end ()) ; let (line , column) = (start . line () , start . column ()) ; :: pretty_assertions :: assert_eq ! (line , $ line1 , "Actual start line: {}" , line) ; :: pretty_assertions :: assert_eq ! (column , $ col1 , "Actual start column: {}" , column) ; let (line , column) = (end . line () , end . column ()) ; :: pretty_assertions :: assert_eq ! (line , $ line2 , "Actual end line: {}" , line) ; :: pretty_assertions :: assert_eq ! (column , $ col2 , "Actual end column: {}" , column) ; } ; }
};
}

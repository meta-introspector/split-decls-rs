// Generated macro for parser_errors_in_submods_are_surfaced (function)
macro_rules! Depcrate_test_parserparser_errors_in_submods_are_surfaced {
() => {
// Module: crate::test::parser
// Provides: {"parser_errors_in_submods_are_surfaced"}
// Dependencies: {}
# [test] fn parser_errors_in_submods_are_surfaced () { let filename = "tests/parser/issue-4126/lib.rs" ; let input_file = PathBuf :: from (filename) ; let exp_mod_name = "invalid" ; let config = read_config (& input_file) ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; if let Err (ErrorKind :: ModuleResolutionError (ModuleResolutionError { module , kind })) = session . format (Input :: File (filename . into ())) { assert_eq ! (& module , exp_mod_name) ; if let ModuleResolutionErrorKind :: ParseError { file : unparsable_file , } = kind { assert_eq ! (unparsable_file , PathBuf :: from ("tests/parser/issue-4126/invalid.rs") ,) ; } else { panic ! ("Expected parser error") ; } } else { panic ! ("Expected ModuleResolution operation error") ; } }
};
}

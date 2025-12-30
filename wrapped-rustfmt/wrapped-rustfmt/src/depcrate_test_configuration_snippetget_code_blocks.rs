// Generated macro for get_code_blocks (function)
macro_rules! Depcrate_test_configuration_snippetget_code_blocks {
() => {
// Module: crate::test::configuration_snippet
// Provides: {"get_code_blocks"}
// Dependencies: {}
fn get_code_blocks () -> Vec < ConfigCodeBlock > { let mut file_iter = BufReader :: new (fs :: File :: open (Path :: new (CONFIGURATIONS_FILE_NAME)) . unwrap_or_else (| _ | panic ! ("couldn't read file {}" , CONFIGURATIONS_FILE_NAME)) ,) . lines () . map (Result :: unwrap) . enumerate () ; let mut code_blocks : Vec < ConfigCodeBlock > = Vec :: new () ; let mut hash_set = Config :: hash_set () ; while let Some (cb) = ConfigCodeBlock :: extract (& mut file_iter , code_blocks . last () , & mut hash_set) { code_blocks . push (cb) ; } for name in hash_set { if ! Config :: is_hidden_option (& name) { panic ! ("{name} does not have a configuration guide") ; } } code_blocks }
};
}

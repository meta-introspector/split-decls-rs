// Generated macro for preprocess (function)
macro_rules! Depcrate_compiler_gccpreprocess {
() => {
// Module: crate::compiler::gcc
// Provides: {"preprocess"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] pub async fn preprocess < F , T > (creator : & T , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , may_dist : bool , kind : CCompilerKind , rewrite_includes_only : bool , ignorable_whitespace_flags : Vec < String > , language_to_arg : F ,) -> Result < process :: Output > where F : Fn (Language) -> Option < & 'static str > , T : CommandCreatorSync , { trace ! ("preprocess") ; let mut cmd = creator . clone () . new_command_sync (executable) ; preprocess_cmd (& mut cmd , parsed_args , cwd , env_vars , may_dist , kind , rewrite_includes_only , ignorable_whitespace_flags , language_to_arg ,) ; if log_enabled ! (Trace) { trace ! ("preprocess: {:?}" , cmd) ; } run_input_output (cmd , None) . await }
};
}

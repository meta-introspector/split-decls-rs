// Generated macro for preprocess (function)
macro_rules! Depcrate_compiler_tasking_vxpreprocess {
() => {
// Module: crate::compiler::tasking_vx
// Provides: {"preprocess"}
// Dependencies: {}
async fn preprocess < T > (creator : & T , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , _may_dist : bool , _rewrite_includes_only : bool ,) -> Result < process :: Output > where T : CommandCreatorSync , { let mut preprocess = creator . clone () . new_command_sync (executable) ; preprocess . arg ("-E") . arg (& parsed_args . input) . args (& parsed_args . preprocessor_args) . args (& parsed_args . common_args) . env_clear () . envs (env_vars . to_vec ()) . current_dir (cwd) ; if log_enabled ! (Trace) { trace ! ("preprocess: {:?}" , preprocess) ; } let preprocess = run_input_output (preprocess , None) ; if let Some (ref depfile) = parsed_args . depfile { let mut generate_depfile = creator . clone () . new_command_sync (executable) ; generate_depfile . arg ("-Em") . arg ("-o") . arg (depfile) . arg (& parsed_args . input) . args (& parsed_args . preprocessor_args) . args (& parsed_args . common_args) . env_clear () . envs (env_vars . to_vec ()) . current_dir (cwd) ; if log_enabled ! (Trace) { trace ! ("dep file generation: {:?}" , generate_depfile) ; } let generate_depfile = run_input_output (generate_depfile , None) ; generate_depfile . and_then (| _ | preprocess) . await } else { preprocess . await } }
};
}

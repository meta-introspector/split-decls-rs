// Generated macro for preprocess (function)
macro_rules! Depcrate_compiler_diabpreprocess {
() => {
// Module: crate::compiler::diab
// Provides: {"preprocess"}
// Dependencies: {}
pub async fn preprocess < T > (creator : & T , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , _may_dist : bool ,) -> Result < process :: Output > where T : CommandCreatorSync , { let mut cmd = creator . clone () . new_command_sync (executable) ; cmd . arg ("-E") . arg (& parsed_args . input) . args (& parsed_args . dependency_args) . args (& parsed_args . preprocessor_args) . args (& parsed_args . common_args) . env_clear () . envs (env_vars . to_vec ()) . current_dir (cwd) ; if log_enabled ! (Trace) { trace ! ("preprocess: {:?}" , cmd) ; } run_input_output (cmd , None) . await }
};
}

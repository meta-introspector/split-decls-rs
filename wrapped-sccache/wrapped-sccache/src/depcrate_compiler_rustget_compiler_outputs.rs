// Generated macro for get_compiler_outputs (function)
macro_rules! Depcrate_compiler_rustget_compiler_outputs {
() => {
// Module: crate::compiler::rust
// Provides: {"get_compiler_outputs"}
// Dependencies: {}
# [doc = " Run `rustc --print file-names` to get the outputs of compilation."] async fn get_compiler_outputs < T > (creator : & T , executable : & Path , arguments : Vec < OsString > , cwd : & Path , env_vars : & [(OsString , OsString)] ,) -> Result < Vec < String > > where T : Clone + CommandCreatorSync , { let mut cmd = creator . clone () . new_command_sync (executable) ; cmd . args (& arguments) . args (& ["--print" , "file-names"]) . env_clear () . envs (env_vars . to_vec ()) . current_dir (cwd) ; if log_enabled ! (Trace) { trace ! ("get_compiler_outputs: {:?}" , cmd) ; } let outputs = run_input_output (cmd , None) . await ? ; let outstr = String :: from_utf8 (outputs . stdout) . context ("Error parsing rustc output") ? ; if log_enabled ! (Trace) { trace ! ("get_compiler_outputs: {:?}" , outstr) ; } Ok (outstr . lines () . map (| l | l . to_owned ()) . collect ()) }
};
}

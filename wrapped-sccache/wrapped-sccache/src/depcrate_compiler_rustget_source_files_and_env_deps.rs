// Generated macro for get_source_files_and_env_deps (function)
macro_rules! Depcrate_compiler_rustget_source_files_and_env_deps {
() => {
// Module: crate::compiler::rust
// Provides: {"get_source_files_and_env_deps"}
// Dependencies: {}
# [doc = " Get absolute paths for all source files and env-deps listed in rustc's dep-info output."] async fn get_source_files_and_env_deps < T > (creator : & T , crate_name : & str , executable : & Path , arguments : & [OsString] , cwd : & Path , env_vars : & [(OsString , OsString)] , pool : & tokio :: runtime :: Handle ,) -> Result < (Vec < PathBuf > , Vec < (OsString , OsString) >) > where T : CommandCreatorSync , { let start = time :: Instant :: now () ; let temp_dir = tempfile :: Builder :: new () . prefix ("sccache") . tempdir () . context ("Failed to create temp dir") ? ; let dep_file = temp_dir . path () . join ("deps.d") ; let mut cmd = creator . clone () . new_command_sync (executable) ; cmd . args (arguments) . args (& ["--emit" , "dep-info"]) . arg ("-o") . arg (& dep_file) . env_clear () . envs (env_vars . to_vec ()) . current_dir (cwd) ; trace ! ("[{}]: get dep-info: {:?}" , crate_name , cmd) ; let _dep_info = run_input_output (cmd , None) . await ? ; let cwd = cwd . to_owned () ; let name2 = crate_name . to_owned () ; let parsed = pool . spawn_blocking (move | | { parse_dep_file (& dep_file , & cwd) . with_context (| | format ! ("Failed to parse dep info for {}" , name2)) }) . await ? ; parsed . map (move | (files , env_deps) | { trace ! ("[{}]: got {} source files and {} env-deps from dep-info in {}" , crate_name , files . len () , env_deps . len () , fmt_duration_as_secs (& start . elapsed ())) ; drop (temp_dir) ; (files , env_deps) }) }
};
}

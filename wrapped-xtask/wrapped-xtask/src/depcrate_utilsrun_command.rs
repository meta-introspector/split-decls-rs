// Generated macro for run_command (function)
macro_rules! Depcrate_utilsrun_command {
() => {
// Module: crate::utils
// Provides: {"run_command"}
// Dependencies: {}
pub fn run_command (program : & str , args : & [& str] , cwd : Option < & str > , envs : & [(& str , & str)] ,) -> anyhow :: Result < () > { let mut cmd = Command :: new (program) ; cmd . args (args) . envs (envs . iter () . copied ()) ; let cwd = if let Some (path) = cwd { cmd . current_dir (path) ; format ! ("{path}$ ") } else { "" . to_string () } ; let cmdline = format ! ("{cwd}{program} {}" , args . join (" ")) ; println ! ("🏃 {cmdline}") ; cmd . status () . map_err (| e | anyhow ! ("could not run '{}': {}" , cmdline , e)) . and_then (| exit_status | match exit_status . success () { true => Ok (()) , false => Err (anyhow ! ("'{}' did not finish successfully: {}" , cmdline , exit_status)) , }) }
};
}

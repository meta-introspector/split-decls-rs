// Generated macro for get_installed (function)
macro_rules! Depcrate_targetsget_installed {
() => {
// Module: crate::targets
// Provides: {"get_installed"}
// Dependencies: {}
# [doc = " Get all currently installed compilation targets"] fn get_installed () -> anyhow :: Result < HashSet < String > > { let stdout = run_capturing_stdout (Command :: new ("rustup") . args (["target" , "list" , "--installed"])) ? ; Ok (stdout . lines () . map (| s | s . to_string ()) . collect ()) }
};
}

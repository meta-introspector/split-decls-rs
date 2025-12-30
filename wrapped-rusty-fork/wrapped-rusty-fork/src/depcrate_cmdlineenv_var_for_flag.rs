// Generated macro for env_var_for_flag (function)
macro_rules! Depcrate_cmdlineenv_var_for_flag {
() => {
// Module: crate::cmdline
// Provides: {"env_var_for_flag"}
// Dependencies: {}
pub (crate) fn env_var_for_flag (flag : & str) -> String { let mut var = "RUSTY_FORK_FLAG_" . to_owned () ; var . push_str (& flag . trim_start_matches ('-') . to_uppercase () . replace ('-' , "_")) ; var }
};
}

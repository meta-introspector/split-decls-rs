// Generated macro for basic_exit_status (function)
macro_rules! Depcrate_cmdbasic_exit_status {
() => {
// Module: crate::cmd
// Provides: {"basic_exit_status"}
// Dependencies: {}
fn basic_exit_status (status : std :: process :: ExitStatus) -> String { if let Some (code) = status . code () { code . to_string () } else { "interrupted" . to_owned () } }
};
}

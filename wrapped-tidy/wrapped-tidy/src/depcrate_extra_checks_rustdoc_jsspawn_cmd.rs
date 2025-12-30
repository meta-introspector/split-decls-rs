// Generated macro for spawn_cmd (function)
macro_rules! Depcrate_extra_checks_rustdoc_jsspawn_cmd {
() => {
// Module: crate::extra_checks::rustdoc_js
// Provides: {"spawn_cmd"}
// Dependencies: {}
fn spawn_cmd (cmd : & mut Command) -> Result < Child , io :: Error > { cmd . spawn () . map_err (| err | { eprintln ! ("unable to run {cmd:?} due to {err:?}") ; err }) }
};
}

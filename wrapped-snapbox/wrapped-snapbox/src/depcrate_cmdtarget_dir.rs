// Generated macro for target_dir (function)
macro_rules! Depcrate_cmdtarget_dir {
() => {
// Module: crate::cmd
// Provides: {"target_dir"}
// Dependencies: {}
fn target_dir () -> std :: path :: PathBuf { std :: env :: current_exe () . ok () . map (| mut path | { path . pop () ; if path . ends_with ("deps") { path . pop () ; } path }) . unwrap () }
};
}

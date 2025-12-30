// Generated macro for select_valid_dryrun_lines (function)
macro_rules! Depcrate_compiler_nvccselect_valid_dryrun_lines {
() => {
// Module: crate::compiler::nvcc
// Provides: {"select_valid_dryrun_lines"}
// Dependencies: {}
fn select_valid_dryrun_lines (re : & Regex , line : & str) -> Result < String > { match re . captures (line) { Some (caps) => { let (_ , [rest]) = caps . extract () ; Ok (rest . to_string ()) } _ => Err (anyhow ! ("nvcc error: {:?}" , line)) , } }
};
}

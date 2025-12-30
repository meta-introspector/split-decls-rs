// Generated macro for format_generated_rust_files (function)
macro_rules! Depcrate_format_generated_rust_filesformat_generated_rust_files {
() => {
// Module: crate::format_generated_rust_files
// Provides: {"format_generated_rust_files"}
// Dependencies: {}
pub fn format_generated_rust_files (output_dir : & Path , verbose : bool) -> Result < () > { if verbose { println ! ("DEBUG: Skipping rustfmt formatting in {} (disabled due to edition issues)" , output_dir . display ()) ; } Ok (()) }
};
}

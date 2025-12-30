// Generated macro for idempotent_check (function)
macro_rules! Depcrate_testidempotent_check {
() => {
// Module: crate::test
// Provides: {"idempotent_check"}
// Dependencies: {}
fn idempotent_check (filename : & PathBuf , opt_config : & Option < PathBuf > ,) -> Result < FormatReport , IdempotentCheckError > { let sig_comments = read_significant_comments (filename) ; let config = if let Some (ref config_file_path) = opt_config { let (edition , style_edition , version) = get_editions_from_comments (& sig_comments) ; Config :: from_toml_path (config_file_path , edition , style_edition , version) . expect ("`rustfmt.toml` not found") } else { read_config (filename) } ; let (parsing_errors , source_file , format_report) = format_file (filename , config) ; if parsing_errors { return Err (IdempotentCheckError :: Parse) ; } let mut write_result = HashMap :: new () ; for (filename , text) in source_file { if let FileName :: Real (ref filename) = filename { write_result . insert (filename . to_owned () , text) ; } } let target = sig_comments . get ("target") . map (| x | & (* x) [..]) ; handle_result (write_result , target) . map (| _ | format_report) }
};
}

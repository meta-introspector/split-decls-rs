// Generated macro for parse_args (function)
macro_rules! Depcrateparse_args {
() => {
// Module: crate
// Provides: {"parse_args"}
// Dependencies: {}
fn parse_args () -> Vec < (PathBuf , Option < PathBuf >) > { let mut args_it = std :: env :: args () . skip (1) ; assert ! (1 <= args_it . len () && args_it . len () <= 2 , "Usage: cargo run -p stdarch-gen-arm -- INPUT_DIR [OUTPUT_DIR]\n\
        where:\n\
        - INPUT_DIR contains a tree like: INPUT_DIR/<feature>/<arch>.spec.yml\n\
        - OUTPUT_DIR is a directory like: crates/core_arch/src/") ; let in_path = Path :: new (args_it . next () . unwrap () . as_str ()) . to_path_buf () ; assert ! (in_path . exists () && in_path . is_dir () , "invalid path {in_path:#?} given") ; let out_dir = if let Some (dir) = args_it . next () { let out_path = Path :: new (dir . as_str ()) . to_path_buf () ; assert ! (out_path . exists () && out_path . is_dir () , "invalid path {out_path:#?} given") ; Some (out_path) } else { std :: env :: current_exe () . map (| mut f | { f . pop () ; f . push ("../../crates/core_arch/src/") ; f . exists () . then_some (f) }) . ok () . flatten () } ; WalkDir :: new (in_path) . into_iter () . filter_map (Result :: ok) . filter (| f | f . file_type () . is_file ()) . map (| f | (f . into_path () , out_dir . clone ())) . collect () }
};
}

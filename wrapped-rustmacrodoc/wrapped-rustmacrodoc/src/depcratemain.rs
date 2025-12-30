// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { let args : Vec < String > = std :: env :: args () . collect () ; let root_path_str = args . get (1) . map_or ("." , | s | s . as_str ()) ; let root_path = PathBuf :: from (root_path_str) ; if ! root_path . exists () { anyhow :: bail ! ("Root path does not exist: {}" , root_path . display ()) ; } let mut all_macros_info = Vec :: new () ; for entry in WalkDir :: new (& root_path) . into_iter () . filter_map (| e | e . ok ()) { let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { let file_content = fs :: read_to_string (path) . with_context (| | format ! ("Failed to read file: {}" , path . display ())) ? ; let syntax_tree = syn :: parse_file (& file_content) . with_context (| | format ! ("Failed to parse file: {}" , path . display ())) ? ; let mut visitor = MacroVisitor { macros : Vec :: new () , file_path : path . to_path_buf () , } ; visitor . visit_file (& syntax_tree) ; all_macros_info . extend (visitor . macros) ; } } let report = MacroReport { macros : all_macros_info , } ; let toml_report = toml :: to_string_pretty (& report) . context ("Failed to serialize macro report to TOML") ? ; println ! ("{}" , toml_report) ; Ok (()) }
};
}

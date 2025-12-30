// Generated macro for exec (function)
macro_rules! Depcrateexec {
() => {
// Module: crate
// Provides: {"exec"}
// Dependencies: {}
pub fn exec (matches : & clap :: ArgMatches) -> Result < () > { let mut args = vec ! [] ; match matches . get_one :: < String > ("color") { Some (c) if matches ! (c . as_str () , "auto" | "always" | "never") => { args . push ("--color") ; args . push (c) ; } Some (c) => { anyhow :: bail ! ("argument for --color must be auto, always, or \
                 never, but found `{}`" , c) ; } _ => { } } if matches . get_flag ("quiet") { args . push ("--quiet") ; } let verbose_count = matches . get_count ("verbose") ; for _ in 0 .. verbose_count { args . push ("--verbose") ; } if matches . get_flag ("write-changes") { args . push ("--write-changes") ; } let metadata = MetadataCommand :: new () . exec () . expect ("cargo_metadata failed") ; let required_version = extract_workflow_typos_version (& metadata) ? ; let outdir = metadata . build_directory . unwrap_or_else (| | metadata . target_directory) . as_std_path () . join ("tmp") ; let workspace_root = metadata . workspace_root . as_path () . as_std_path () ; let bin_path = crate :: ensure_version_or_cargo_install (& outdir , required_version) ? ; eprintln ! ("running {BIN_NAME}") ; Command :: new (bin_path) . current_dir (workspace_root) . args (args) . status () ? ; Ok (()) }
};
}

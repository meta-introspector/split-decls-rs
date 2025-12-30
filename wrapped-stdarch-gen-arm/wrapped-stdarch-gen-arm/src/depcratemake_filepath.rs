// Generated macro for make_filepath (function)
macro_rules! Depcratemake_filepath {
() => {
// Module: crate
// Provides: {"make_filepath"}
// Dependencies: {}
fn make_filepath < F : FnOnce (& str) -> String > (in_filepath : & Path , out_dirpath : & Path , name_formatter : F ,) -> PathBuf { let mut parts = in_filepath . components () . rev () . map (| f | { f . as_os_str () . to_str () . expect ("Inputs must have valid, UTF-8 file_name()") }) ; let yml = parts . next () . expect ("Not enough input path elements.") ; let feature = parts . next () . expect ("Not enough input path elements.") ; let arch = yml . strip_suffix (".yml") . expect ("Expected .yml file input.") . strip_suffix (".spec") . expect ("Expected .spec.yml file input.") ; if arch . is_empty () { panic ! ("Extended ARCH.spec.yml file input.") ; } let mut output = out_dirpath . to_path_buf () ; output . push (arch) ; output . push (feature) ; output . push (name_formatter (arch)) ; output }
};
}

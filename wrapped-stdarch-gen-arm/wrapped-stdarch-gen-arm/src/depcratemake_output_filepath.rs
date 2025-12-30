// Generated macro for make_output_filepath (function)
macro_rules! Depcratemake_output_filepath {
() => {
// Module: crate
// Provides: {"make_output_filepath"}
// Dependencies: {}
# [doc = " Derive an output file path from an input file path and an output directory."] # [doc = ""] # [doc = " `in_filepath` is expected to have a structure like:"] # [doc = "     .../<feature>/<arch>.spec.yml"] # [doc = ""] # [doc = " The resulting output path will have a structure like:"] # [doc = "     <out_dirpath>/<arch>/<feature>/generated.rs"] # [doc = ""] # [doc = " Panics if the resulting name is empty, or if file_name() is not UTF-8."] fn make_output_filepath (in_filepath : & Path , out_dirpath : & Path) -> PathBuf { make_filepath (in_filepath , out_dirpath , | _name : & str | { "generated.rs" . to_owned () }) }
};
}

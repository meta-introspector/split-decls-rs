// Generated macro for impl_393 (impl)
macro_rules! Depcrate_config_file_linesimpl_393 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_393"}
// Dependencies: {}
impl JsonSpan { fn into_tuple (self) -> Result < (FileName , Range) , FileLinesError > { let (lo , hi) = self . range ; let canonical = canonicalize_path_string (& self . file) . ok_or (FileLinesError :: CannotCanonicalize (self . file)) ? ; Ok ((canonical , Range :: new (lo , hi))) } }
};
}

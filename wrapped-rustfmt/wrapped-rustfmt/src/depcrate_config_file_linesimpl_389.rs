// Generated macro for impl_389 (impl)
macro_rules! Depcrate_config_file_linesimpl_389 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_389"}
// Dependencies: {}
impl str :: FromStr for FileLines { type Err = FileLinesError ; fn from_str (s : & str) -> Result < FileLines , Self :: Err > { let v : Vec < JsonSpan > = json :: from_str (s) . map_err (FileLinesError :: Json) ? ; let mut m = HashMap :: new () ; for js in v { let (s , r) = JsonSpan :: into_tuple (js) ? ; m . entry (s) . or_insert_with (Vec :: new) . push (r) ; } Ok (FileLines :: from_ranges (m)) } }
};
}

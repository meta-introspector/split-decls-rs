// Generated macro for impl_316 (impl)
macro_rules! Depcrate_special_casingimpl_316 {
() => {
// Module: crate::special_casing
// Provides: {"impl_316"}
// Dependencies: {}
impl std :: str :: FromStr for SpecialCaseMapping { type Err = Error ; fn from_str (line : & str) -> Result < SpecialCaseMapping , Error > { let re_parts = regex ! (r"(?x)
                ^
                \s*(?P<codepoint>[^\s;]+)\s*;
                \s*(?P<lower>[^;]+)\s*;
                \s*(?P<title>[^;]+)\s*;
                \s*(?P<upper>[^;]+)\s*;
                \s*(?P<conditions>[^;\x23]+)?
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid SpecialCasing line: '{}'" , line) , } ; let conditions = caps . name ("conditions") . map (| x | { x . as_str () . trim () . split_whitespace () . map (| c | c . to_string ()) . collect () }) . unwrap_or (vec ! []) ; Ok (SpecialCaseMapping { codepoint : caps ["codepoint"] . parse () ? , lowercase : parse_codepoint_sequence (& caps ["lower"]) ? , titlecase : parse_codepoint_sequence (& caps ["title"]) ? , uppercase : parse_codepoint_sequence (& caps ["upper"]) ? , conditions , }) } }
};
}

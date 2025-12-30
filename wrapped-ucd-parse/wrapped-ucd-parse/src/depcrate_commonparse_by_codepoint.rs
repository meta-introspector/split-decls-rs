// Generated macro for parse_by_codepoint (function)
macro_rules! Depcrate_commonparse_by_codepoint {
() => {
// Module: crate::common
// Provides: {"parse_by_codepoint"}
// Dependencies: {}
# [doc = " Parse a particular file in the UCD into a map from codepoint to the record."] # [doc = ""] # [doc = " The given directory should be the directory to the UCD."] pub fn parse_by_codepoint < P , D > (ucd_dir : P ,) -> Result < BTreeMap < Codepoint , D > , Error > where P : AsRef < Path > , D : UcdFileByCodepoint , { let mut map = BTreeMap :: new () ; for result in D :: from_dir (ucd_dir) ? { let x = result ? ; for cp in x . codepoints () { map . insert (cp , x . clone ()) ; } } Ok (map) }
};
}

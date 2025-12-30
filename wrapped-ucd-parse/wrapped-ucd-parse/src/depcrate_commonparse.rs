// Generated macro for parse (function)
macro_rules! Depcrate_commonparse {
() => {
// Module: crate::common
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse a particular file in the UCD into a sequence of rows."] # [doc = ""] # [doc = " The given directory should be the directory to the UCD."] pub fn parse < P , D > (ucd_dir : P) -> Result < Vec < D > , Error > where P : AsRef < Path > , D : UcdFile , { let mut xs = vec ! [] ; for result in D :: from_dir (ucd_dir) ? { let x = result ? ; xs . push (x) ; } Ok (xs) }
};
}

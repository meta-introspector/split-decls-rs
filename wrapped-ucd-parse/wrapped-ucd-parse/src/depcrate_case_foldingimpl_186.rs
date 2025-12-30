// Generated macro for impl_186 (impl)
macro_rules! Depcrate_case_foldingimpl_186 {
() => {
// Module: crate::case_folding
// Provides: {"impl_186"}
// Dependencies: {}
impl std :: str :: FromStr for CaseFold { type Err = Error ; fn from_str (line : & str) -> Result < CaseFold , Error > { let re_parts = regex ! (r"(?x)
                ^
                \s*(?P<codepoint>[^\s;]+)\s*;
                \s*(?P<status>[^\s;]+)\s*;
                \s*(?P<mapping>[^;]+)\s*;
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid CaseFolding line: '{}'" , line) , } ; let mut mapping = vec ! [] ; for cp in caps ["mapping"] . split_whitespace () { mapping . push (cp . parse () ?) ; } Ok (CaseFold { codepoint : caps ["codepoint"] . parse () ? , status : caps ["status"] . parse () ? , mapping , }) } }
};
}

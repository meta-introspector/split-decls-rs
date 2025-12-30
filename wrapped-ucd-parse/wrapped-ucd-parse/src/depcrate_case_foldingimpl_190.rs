// Generated macro for impl_190 (impl)
macro_rules! Depcrate_case_foldingimpl_190 {
() => {
// Module: crate::case_folding
// Provides: {"impl_190"}
// Dependencies: {}
impl std :: str :: FromStr for CaseStatus { type Err = Error ; fn from_str (s : & str) -> Result < CaseStatus , Error > { match s { "C" => Ok (CaseStatus :: Common) , "F" => Ok (CaseStatus :: Full) , "S" => Ok (CaseStatus :: Simple) , "T" => Ok (CaseStatus :: Special) , _ => err ! ("unrecognized case status: '{}' \
                 (must be one of C, F, S or T)" , s) , } } }
};
}

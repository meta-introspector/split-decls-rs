// Generated macro for CaseFold (struct)
macro_rules! Depcrate_case_foldingCaseFold {
() => {
// Module: crate::case_folding
// Provides: {"CaseFold"}
// Dependencies: {}
# [doc = " A single row in the `CaseFolding.txt` file."] # [doc = ""] # [doc = " The contents of `CaseFolding.txt` are a convenience derived from both"] # [doc = " `UnicodeData.txt` and `SpecialCasing.txt`."] # [doc = ""] # [doc = " Note that a single codepoint may be mapped multiple times. In particular,"] # [doc = " a single codepoint might have distinct `CaseStatus::Simple` and"] # [doc = " `CaseStatus::Full` mappings."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct CaseFold { # [doc = " The codepoint that is being mapped."] pub codepoint : Codepoint , # [doc = " The case status of this mapping."] pub status : CaseStatus , # [doc = " The actual case mapping, which is more than one codepoint if this is"] # [doc = " a \"full\" mapping."] pub mapping : Vec < Codepoint > , }
};
}

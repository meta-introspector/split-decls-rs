// Generated macro for CaseStatus (enum)
macro_rules! Depcrate_case_foldingCaseStatus {
() => {
// Module: crate::case_folding
// Provides: {"CaseStatus"}
// Dependencies: {}
# [doc = " The status of a particular case mapping."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum CaseStatus { # [doc = " Case mappings shared by both \"simple\" and \"full\" mappings."] Common , # [doc = " A case mapping that changes the number of codepoints."] Full , # [doc = " A case mapping that doesn't change the number of codepoints, when it"] # [doc = " differs from `Full`."] Simple , # [doc = " Special cases (currently only for Turkic mappings) that are typically"] # [doc = " excluded by default. Special cases don't change the number of"] # [doc = " codepoints, but may changed the encoding (e.g., UTF-8) length in bytes."] Special , }
};
}

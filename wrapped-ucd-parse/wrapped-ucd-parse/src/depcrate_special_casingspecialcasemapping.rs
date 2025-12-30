// Generated macro for SpecialCaseMapping (struct)
macro_rules! Depcrate_special_casingSpecialCaseMapping {
() => {
// Module: crate::special_casing
// Provides: {"SpecialCaseMapping"}
// Dependencies: {}
# [doc = " A single row in the `SpecialCasing.txt` file."] # [doc = ""] # [doc = " Note that a single codepoint may be mapped multiple times. In particular,"] # [doc = " a single codepoint might have mappings based on distinct language sensitive"] # [doc = " conditions (e.g., `U+0307`)."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct SpecialCaseMapping { # [doc = " The codepoint that is being mapped."] pub codepoint : Codepoint , # [doc = " The lowercase mapping, which may be empty."] pub lowercase : Vec < Codepoint > , # [doc = " The titlecase mapping, which may be empty."] pub titlecase : Vec < Codepoint > , # [doc = " The uppercase mapping, which may be empty."] pub uppercase : Vec < Codepoint > , # [doc = " A list of language specific conditions, see `SpecialCasing.txt` for"] # [doc = " more details."] pub conditions : Vec < String > , }
};
}

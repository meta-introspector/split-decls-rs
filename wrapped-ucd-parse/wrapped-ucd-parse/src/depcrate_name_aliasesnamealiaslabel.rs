// Generated macro for NameAliasLabel (enum)
macro_rules! Depcrate_name_aliasesNameAliasLabel {
() => {
// Module: crate::name_aliases
// Provides: {"NameAliasLabel"}
// Dependencies: {}
# [doc = " The label of a name alias."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum NameAliasLabel { # [doc = " Corrections for serious problems in a character name."] Correction , # [doc = " ISO 6429 names for C0 and C1 control functions and other commonly"] # [doc = " occurring names for control codes."] Control , # [doc = " A few widely used alternate names for format characters."] Alternate , # [doc = " Several documented labels for C1 control code points which were"] # [doc = " never actually approved in any standard."] Figment , # [doc = " Commonly occurring abbreviations (or acronyms) for control codes,"] # [doc = " format characters, spaces and variation selectors."] Abbreviation , }
};
}

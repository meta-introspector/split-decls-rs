// Generated macro for PersonNames (struct)
macro_rules! Depcrate_cldr_serde_personnames_person_name_format_json_structPersonNames {
() => {
// Module: crate::cldr_serde::personnames::person_name_format_json_struct
// Provides: {"PersonNames"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct PersonNames { # [serde (rename = "givenFirst")] pub (crate) given_first : Vec < String > , # [serde (rename = "surnameFirst")] pub (crate) surname_first : Vec < String > , # [serde (rename = "foreignSpaceReplacement")] pub (crate) foreign_space_replacement : String , pub (crate) initial : String , # [serde (rename = "initialSequence")] pub (crate) initial_sequence : String , # [serde (rename = "personName")] pub (crate) formatting_pattern : OrderFormatting , }
};
}

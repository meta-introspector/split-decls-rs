// Generated macro for LengthPattern (enum)
macro_rules! Depcrate_cldr_serde_caLengthPattern {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"LengthPattern"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize , Clone)] # [serde (untagged)] pub (crate) enum LengthPattern { Plain (String) , WithNumberingSystems { # [serde (rename = "_value")] pattern : String , # [serde (rename = "_numbers")] numbering_systems : String , } , }
};
}

// Generated macro for Eras (struct)
macro_rules! Depcrate_cldr_serde_caEras {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"Eras"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize , Clone , Default)] pub (crate) struct Eras { # [serde (rename = "eraNames")] pub (crate) names : HashMap < String , String > , # [serde (rename = "eraAbbr")] pub (crate) abbr : HashMap < String , String > , # [serde (rename = "eraNarrow")] pub (crate) narrow : HashMap < String , String > , }
};
}

// Generated macro for Bcp47TzidAliasData (struct)
macro_rules! Depcrate_cldr_serde_time_zones_bcp47_tzidBcp47TzidAliasData {
() => {
// Module: crate::cldr_serde::time_zones::bcp47_tzid
// Provides: {"Bcp47TzidAliasData"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone , Deserialize)] pub (crate) struct Bcp47TzidAliasData { # [serde (rename = "_deprecated")] pub (crate) deprecated : Option < bool > , # [serde (rename = "_preferred")] pub (crate) preferred : Option < TimeZone > , # [serde (rename = "_description")] pub (crate) description : String , # [serde (rename = "_alias")] pub (crate) alias : Option < String > , # [serde (rename = "_since")] pub (crate) since : Option < String > , # [serde (rename = "_iana")] pub (crate) iana : Option < String > , # [serde (rename = "_region")] pub (crate) region : Option < Region > , }
};
}

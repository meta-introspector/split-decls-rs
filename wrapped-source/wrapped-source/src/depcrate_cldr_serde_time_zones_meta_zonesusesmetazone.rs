// Generated macro for UsesMetazone (struct)
macro_rules! Depcrate_cldr_serde_time_zones_meta_zonesUsesMetazone {
() => {
// Module: crate::cldr_serde::time_zones::meta_zones
// Provides: {"UsesMetazone"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone , Deserialize)] pub (crate) struct UsesMetazone { # [serde (rename = "_mzone")] pub (crate) mzone : Option < String > , # [serde (rename = "_from" , default , deserialize_with = "deserialize_date")] pub (crate) from : Option < Timestamp > , # [serde (rename = "_to" , default , deserialize_with = "deserialize_date")] pub (crate) to : Option < Timestamp > , # [serde (rename = "_stdOffset")] pub (crate) std_offset : Option < String > , # [serde (rename = "_dstOffset")] pub (crate) dst_offset : Option < String > , }
};
}

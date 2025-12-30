// Generated macro for ZoneInfo64Raw (struct)
macro_rules! Depcrate_deserializeZoneInfo64Raw {
() => {
// Module: crate::deserialize
// Provides: {"ZoneInfo64Raw"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (rename = "zoneinfo64")] # [serde (rename_all = "PascalCase")] struct ZoneInfo64Raw < 'a > { # [serde (borrow)] zones : Vec < TzZoneRaw < 'a > > , # [serde (borrow , deserialize_with = "resb::binary::helpers::vec_utf_16")] names : Vec < & 'a PotentialUtf16 > , # [serde (borrow , deserialize_with = "rules")] rules : Vec < (& 'a str , TzRule) > , # [serde (deserialize_with = "regions")] regions : Vec < Region > , }
};
}

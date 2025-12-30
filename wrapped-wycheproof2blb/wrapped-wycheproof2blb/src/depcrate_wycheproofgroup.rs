// Generated macro for Group (struct)
macro_rules! Depcrate_wycheproofGroup {
() => {
// Module: crate::wycheproof
// Provides: {"Group"}
// Dependencies: {}
# [doc = " `Group` represents the common elements of a testGroups object in a Wycheproof suite."] # [doc = " Implementations should embed (using `#[serde(flatten)]`) Group in a struct that"] # [doc = " strongly types its list of cases."] # [derive (Debug , Deserialize)] pub struct Group { # [serde (rename = "type")] pub group_type : String , }
};
}

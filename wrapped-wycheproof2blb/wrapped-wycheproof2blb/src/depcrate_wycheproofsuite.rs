// Generated macro for Suite (struct)
macro_rules! Depcrate_wycheproofSuite {
() => {
// Module: crate::wycheproof
// Provides: {"Suite"}
// Dependencies: {}
# [doc = " `Suite` represents the common elements of the top level object in a Wycheproof json"] # [doc = " file.  Implementations should embed (using `#[serde(flatten)]`) `Suite` in a struct"] # [doc = " that strongly types the `testGroups` field."] # [derive (Debug , Deserialize)] pub struct Suite { pub algorithm : String , # [serde (rename = "generatorVersion")] pub generator_version : String , # [serde (rename = "numberOfTests")] pub number_of_tests : i32 , pub notes : std :: collections :: HashMap < String , String > , }
};
}

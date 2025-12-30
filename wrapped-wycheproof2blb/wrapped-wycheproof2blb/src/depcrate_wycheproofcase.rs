// Generated macro for Case (struct)
macro_rules! Depcrate_wycheproofCase {
() => {
// Module: crate::wycheproof
// Provides: {"Case"}
// Dependencies: {}
# [doc = " `Case` represents the common elements of a tests object in a Wycheproof group."] # [doc = " Implementations should embed (using `#[serde(flatten)]`) `Case` in a struct that"] # [doc = " contains fields specific to the test type."] # [derive (Debug , Deserialize)] pub struct Case { # [serde (rename = "tcId")] pub case_id : i32 , pub comment : String , # [serde (with = "case_result")] pub result : CaseResult , # [serde (default)] pub flags : Vec < String > , }
};
}

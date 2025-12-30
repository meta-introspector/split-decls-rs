// Generated macro for Branch (struct)
macro_rules! DepcrateBranch {
() => {
// Module: crate
// Provides: {"Branch"}
// Dependencies: {}
# [doc = " A struct to hold some data from the GitHub Branch API."] # [doc = ""] # [doc = " Note how we don't have to define every member -- serde will ignore extra"] # [doc = " data when deserializing"] # [derive (Debug , Serialize , Deserialize)] pub struct Branch { pub name : String , pub commit : Commit , }
};
}

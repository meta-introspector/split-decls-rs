// Generated macro for Package (struct)
macro_rules! Depcrate_manifestPackage {
() => {
// Module: crate::manifest
// Provides: {"Package"}
// Dependencies: {}
# [derive (Serialize , Debug)] pub (crate) struct Package { pub name : String , pub version : String , pub edition : Edition , # [serde (skip_serializing_if = "Option::is_none")] pub resolver : Option < String > , pub publish : bool , }
};
}

// Generated macro for Workspace (struct)
macro_rules! Depcrate_manifestWorkspace {
() => {
// Module: crate::manifest
// Provides: {"Workspace"}
// Dependencies: {}
# [derive (Serialize , Debug)] pub (crate) struct Workspace { # [serde (skip_serializing_if = "Map::is_empty")] pub dependencies : Map < String , Dependency > , }
};
}

// Generated macro for WorkspaceWorkspace (struct)
macro_rules! Depcrate_dependenciesWorkspaceWorkspace {
() => {
// Module: crate::dependencies
// Provides: {"WorkspaceWorkspace"}
// Dependencies: {}
# [derive (Deserialize , Default , Debug)] pub (crate) struct WorkspaceWorkspace { # [serde (default)] pub package : WorkspacePackage , # [serde (default)] pub dependencies : Map < String , Dependency > , }
};
}

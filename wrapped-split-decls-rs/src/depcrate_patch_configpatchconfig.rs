// Generated macro for PatchConfig (struct)
macro_rules! Depcrate_patch_configPatchConfig {
() => {
// Module: crate::patch_config
// Provides: {"PatchConfig"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize)] pub struct PatchConfig { # [serde (default)] pub generated_workspace_member : Vec < GeneratedWorkspaceMember > , # [serde (default)] pub generated_workspace_dependency : Vec < GeneratedWorkspaceDependency > , # [serde (default)] pub generated_crate_dependency : Vec < GeneratedCrateDependency > , }
};
}

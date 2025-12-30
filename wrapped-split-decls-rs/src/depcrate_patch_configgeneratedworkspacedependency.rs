// Generated macro for GeneratedWorkspaceDependency (struct)
macro_rules! Depcrate_patch_configGeneratedWorkspaceDependency {
() => {
// Module: crate::patch_config
// Provides: {"GeneratedWorkspaceDependency"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize)] pub struct GeneratedWorkspaceDependency { pub name : String , pub project_root_path : Option < PathBuf > , pub version : Option < String > , pub features : Option < Vec < String > > , pub package : Option < String > , pub is_patch : Option < bool > , }
};
}

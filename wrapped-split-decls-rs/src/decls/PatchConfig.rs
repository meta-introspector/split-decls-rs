// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PatchConfig",
decl_type: "function",
source_file: "./src/patch_config.rs",
source_crate: ".",
deps: ["GeneratedCrateDependency", "GeneratedWorkspaceMember", "GeneratedWorkspaceDependency"],
uses: ["Vec", "GeneratedCrateDependency", "Debug", "GeneratedWorkspaceMember", "PatchConfig", "Serialize", "GeneratedWorkspaceDependency", "Deserialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        GeneratedCrateDependency!();
        GeneratedWorkspaceMember!();
        GeneratedWorkspaceDependency!();
    };
}

macro_rules! PatchConfig {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize)] pub struct PatchConfig { # [serde (default)] pub generated_workspace_member : Vec < GeneratedWorkspaceMember > , # [serde (default)] pub generated_workspace_dependency : Vec < GeneratedWorkspaceDependency > , # [serde (default)] pub generated_crate_dependency : Vec < GeneratedCrateDependency > , }
    };
}

PatchConfig!();
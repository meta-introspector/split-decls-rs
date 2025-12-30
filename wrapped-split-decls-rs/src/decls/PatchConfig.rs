// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PatchConfig",
decl_type: "function",
source_file: "./src/patch_config.rs",
source_crate: ".",
deps: ["GeneratedWorkspaceMember", "GeneratedCrateDependency", "GeneratedWorkspaceDependency"],
uses: ["GeneratedWorkspaceMember", "GeneratedCrateDependency", "GeneratedWorkspaceDependency", "Serialize", "Debug", "PatchConfig", "Vec", "Deserialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        GeneratedWorkspaceMember!();
        GeneratedCrateDependency!();
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
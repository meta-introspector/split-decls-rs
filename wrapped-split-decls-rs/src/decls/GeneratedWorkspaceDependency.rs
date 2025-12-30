// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "GeneratedWorkspaceDependency",
decl_type: "function",
source_file: "./src/patch_config.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "String", "Vec", "PathBuf", "Deserialize", "GeneratedWorkspaceDependency", "Option", "Debug"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! GeneratedWorkspaceDependency {
    () => {
        # [derive (Debug , Deserialize , Serialize)] pub struct GeneratedWorkspaceDependency { pub name : String , pub project_root_path : Option < PathBuf > , pub version : Option < String > , pub features : Option < Vec < String > > , pub package : Option < String > , pub is_patch : Option < bool > , }
    };
}

GeneratedWorkspaceDependency!();
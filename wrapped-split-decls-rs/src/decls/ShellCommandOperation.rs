// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ShellCommandOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: [],
uses: ["ShellCommandOperation", "Clone", "String", "Deserialize", "Debug", "Serialize", "Option"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ShellCommandOperation {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct ShellCommandOperation { # [serde (rename = "type")] pub op_type : String , pub command : String , # [serde (default)] pub working_dir : Option < String > , # [serde (default = "default_true")] pub capture_output : bool , # [serde (default)] pub error_on_failure : bool , }
    };
}

ShellCommandOperation!();
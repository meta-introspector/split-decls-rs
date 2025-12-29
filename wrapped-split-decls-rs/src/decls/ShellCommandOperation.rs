// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ShellCommandOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Option", "Deserialize", "String", "ShellCommandOperation", "Clone", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! ShellCommandOperation {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct ShellCommandOperation { # [serde (rename = "type")] pub op_type : String , pub command : String , # [serde (default)] pub working_dir : Option < String > , # [serde (default = "default_true")] pub capture_output : bool , # [serde (default)] pub error_on_failure : bool , }
    };
}

ShellCommandOperation!();
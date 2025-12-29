// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "FunctionCallOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: [],
uses: ["Option", "Deserialize", "Vec", "String", "Serialize", "Clone", "FunctionCallOperation", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! FunctionCallOperation {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct FunctionCallOperation { # [serde (rename = "type")] pub op_type : String , pub function : String , # [serde (default)] pub args : Vec < String > , # [serde (default)] pub recursive : Option < bool > , # [serde (default)] pub output_base : Option < String > , }
    };
}

FunctionCallOperation!();
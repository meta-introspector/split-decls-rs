// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "FunctionCallOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "Option", "Debug", "Vec", "Serialize", "String", "FunctionCallOperation", "Deserialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! FunctionCallOperation {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct FunctionCallOperation { # [serde (rename = "type")] pub op_type : String , pub function : String , # [serde (default)] pub args : Vec < String > , # [serde (default)] pub recursive : Option < bool > , # [serde (default)] pub output_base : Option < String > , }
    };
}

FunctionCallOperation!();
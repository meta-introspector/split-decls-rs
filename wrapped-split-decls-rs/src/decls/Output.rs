// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Output",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: [],
uses: ["String", "Debug", "Serialize", "Clone", "Output", "Deserialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! Output {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Output { pub name : String , # [serde (rename = "type")] pub output_type : String , pub description : String , }
    };
}

Output!();
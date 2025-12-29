// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Input",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: [],
uses: ["String", "Deserialize", "Serialize", "Input", "Option", "Debug", "Clone"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! Input {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Input { pub name : String , # [serde (default)] pub from_stage : Option < String > , # [serde (default)] pub from_task : Option < String > , pub output_name : String , }
    };
}

Input!();
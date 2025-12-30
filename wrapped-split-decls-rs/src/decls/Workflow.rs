// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Workflow",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Stage"],
uses: ["Deserialize", "Clone", "Vec", "Stage", "Debug", "Workflow", "String", "Serialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Stage!();
    };
}

macro_rules! Workflow {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Workflow { pub name : String , pub description : String , # [serde (default)] pub style_influences : Vec < String > , pub stages : Vec < Stage > , }
    };
}

Workflow!();
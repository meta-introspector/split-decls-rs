// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Stage",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Input", "Operation", "Output", "Task"],
uses: ["Stage", "Deserialize", "Input", "Debug", "Operation", "Clone", "Serialize", "Output", "Vec", "Task", "Option", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Input!();
        Operation!();
        Output!();
        Task!();
    };
}

macro_rules! Stage {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Stage { pub name : String , pub description : String , # [serde (default)] pub processor_hint : Option < String > , # [serde (default)] pub inputs : Vec < Input > , # [serde (default)] pub outputs : Vec < Output > , pub operation : Operation , # [serde (default)] pub tasks : Vec < Task > , }
    };
}

Stage!();
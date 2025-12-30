// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Stage",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Output", "Task", "Operation", "Input"],
uses: ["Stage", "Serialize", "Option", "Output", "Task", "Operation", "Debug", "Vec", "Deserialize", "Clone", "Input", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Output!();
        Task!();
        Operation!();
        Input!();
    };
}

macro_rules! Stage {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Stage { pub name : String , pub description : String , # [serde (default)] pub processor_hint : Option < String > , # [serde (default)] pub inputs : Vec < Input > , # [serde (default)] pub outputs : Vec < Output > , pub operation : Operation , # [serde (default)] pub tasks : Vec < Task > , }
    };
}

Stage!();
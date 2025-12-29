// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Task",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Operation", "Input", "Output"],
uses: ["Vec", "Serialize", "Operation", "Task", "Debug", "Input", "Deserialize", "Clone", "Output", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Operation!();
        Input!();
        Output!();
    };
}

macro_rules! Task {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Task { pub name : String , pub operation : Operation , # [serde (default)] pub inputs : Vec < Input > , # [serde (default)] pub outputs : Vec < Output > , }
    };
}

Task!();
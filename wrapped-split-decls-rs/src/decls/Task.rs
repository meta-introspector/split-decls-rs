// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Task",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Operation", "Output", "Input"],
uses: ["Deserialize", "Operation", "Output", "Serialize", "Input", "Task", "String", "Vec", "Debug", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Operation!();
        Output!();
        Input!();
    };
}

macro_rules! Task {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Task { pub name : String , pub operation : Operation , # [serde (default)] pub inputs : Vec < Input > , # [serde (default)] pub outputs : Vec < Output > , }
    };
}

Task!();
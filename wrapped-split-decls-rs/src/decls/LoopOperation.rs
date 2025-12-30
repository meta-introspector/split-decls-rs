// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "LoopOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Task"],
uses: ["Deserialize", "LoopOperation", "Serialize", "Vec", "Clone", "String", "Debug", "Task"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! LoopOperation {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct LoopOperation { # [serde (rename = "type")] pub op_type : String , pub over : String , pub loop_var : String , pub tasks : Vec < Task > , }
    };
}

LoopOperation!();
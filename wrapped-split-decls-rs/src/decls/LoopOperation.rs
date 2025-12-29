// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "LoopOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Task"],
uses: ["Debug", "Serialize", "String", "Vec", "Task", "LoopOperation", "Deserialize", "Clone"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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
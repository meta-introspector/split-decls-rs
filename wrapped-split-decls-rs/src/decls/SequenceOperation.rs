// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SequenceOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Task"],
uses: ["Serialize", "SequenceOperation", "Vec", "String", "Deserialize", "Task", "Debug", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! SequenceOperation {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct SequenceOperation { # [serde (rename = "type")] pub op_type : String , pub tasks : Vec < Task > , }
    };
}

SequenceOperation!();
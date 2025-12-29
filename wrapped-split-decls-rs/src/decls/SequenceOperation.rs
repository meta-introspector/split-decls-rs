// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SequenceOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Task"],
uses: ["Task", "Vec", "Serialize", "Deserialize", "SequenceOperation", "Clone", "String", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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
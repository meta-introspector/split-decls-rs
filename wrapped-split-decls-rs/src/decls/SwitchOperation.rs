// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SwitchOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Task"],
uses: ["Serialize", "Debug", "Deserialize", "SwitchOperation", "Clone", "Vec", "Task", "Option", "String", "HashMap"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! SwitchOperation {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct SwitchOperation { # [serde (rename = "type")] pub op_type : String , pub match_on : String , pub cases : HashMap < String , Vec < Task > > , # [serde (default)] pub default : Option < Vec < Task > > , }
    };
}

SwitchOperation!();
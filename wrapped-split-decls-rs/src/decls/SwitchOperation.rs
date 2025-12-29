// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SwitchOperation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Task"],
uses: ["Task", "Clone", "Option", "Deserialize", "String", "Debug", "SwitchOperation", "Vec", "HashMap", "Serialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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
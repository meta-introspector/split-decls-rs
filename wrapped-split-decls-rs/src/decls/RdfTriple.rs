// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RdfTriple",
decl_type: "function",
source_file: "./src/macro_interpreter.rs",
source_crate: ".",
deps: [],
uses: ["RdfTriple", "String", "Debug"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! RdfTriple {
    () => {
        # [derive (Debug)] pub struct RdfTriple { pub subject : String , pub predicate : String , pub object : String , pub timestamp : u64 , }
    };
}

RdfTriple!();
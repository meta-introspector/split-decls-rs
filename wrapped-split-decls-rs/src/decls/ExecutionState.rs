// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ExecutionState",
decl_type: "function",
source_file: "./src/macro_interpreter.rs",
source_crate: ".",
deps: [],
uses: ["ExecutionState", "Vec", "Debug", "Default", "String", "HashMap", "Option"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ExecutionState {
    () => {
        # [derive (Debug , Default)] pub struct ExecutionState { pub current_function : Option < String > , pub call_stack : Vec < String > , pub data_captured : HashMap < String , String > , }
    };
}

ExecutionState!();
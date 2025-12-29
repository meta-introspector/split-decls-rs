// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RdfStateMachine",
decl_type: "function",
source_file: "./src/macro_interpreter.rs",
source_crate: ".",
deps: ["RdfTriple", "ExecutionState"],
uses: ["Vec", "RdfTriple", "ExecutionState", "Debug", "Default", "RdfStateMachine"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        RdfTriple!();
        ExecutionState!();
    };
}

macro_rules! RdfStateMachine {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct RdfStateMachine { pub triples : Vec < RdfTriple > , pub execution_state : ExecutionState , }
    };
}

RdfStateMachine!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RdfStateMachine",
decl_type: "function",
source_file: "./src/macro_interpreter.rs",
source_crate: ".",
deps: ["ExecutionState", "RdfTriple"],
uses: ["ExecutionState", "Default", "RdfTriple", "Vec", "RdfStateMachine", "Debug"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ExecutionState!();
        RdfTriple!();
    };
}

macro_rules! RdfStateMachine {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct RdfStateMachine { pub triples : Vec < RdfTriple > , pub execution_state : ExecutionState , }
    };
}

RdfStateMachine!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BootstrapTrace",
decl_type: "function",
source_file: "./src/bootstrap_tracer.rs",
source_crate: ".",
deps: ["ExecutionState", "RdfTriple"],
uses: ["Debug", "Serialize", "HashMap", "Vec", "BootstrapTrace", "ExecutionState", "String", "Deserialize", "RdfTriple"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ExecutionState!();
        RdfTriple!();
    };
}

macro_rules! BootstrapTrace {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct BootstrapTrace { pub states : HashMap < String , ExecutionState > , pub transitions : Vec < (String , String) > , pub rdf_graph : Vec < RdfTriple > , }
    };
}

BootstrapTrace!();
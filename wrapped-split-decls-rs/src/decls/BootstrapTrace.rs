// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BootstrapTrace",
decl_type: "function",
source_file: "./src/bootstrap_tracer.rs",
source_crate: ".",
deps: ["RdfTriple", "ExecutionState"],
uses: ["HashMap", "Debug", "BootstrapTrace", "String", "RdfTriple", "Vec", "Serialize", "Deserialize", "ExecutionState"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        RdfTriple!();
        ExecutionState!();
    };
}

macro_rules! BootstrapTrace {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct BootstrapTrace { pub states : HashMap < String , ExecutionState > , pub transitions : Vec < (String , String) > , pub rdf_graph : Vec < RdfTriple > , }
    };
}

BootstrapTrace!();
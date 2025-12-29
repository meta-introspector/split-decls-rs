// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SparqlProbeGenerator",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: [],
uses: ["SparqlProbeGenerator", "HashMap", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SparqlProbeGenerator {
    () => {
        pub struct SparqlProbeGenerator { rdf_data : HashMap < String , f64 > , frequency_data : HashMap < String , usize > , }
    };
}

SparqlProbeGenerator!();
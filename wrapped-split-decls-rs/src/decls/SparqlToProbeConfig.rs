// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SparqlToProbeConfig",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: ["SparqlQuery", "ProbeTemplate"],
uses: ["String", "SparqlQuery", "Debug", "ProbeTemplate", "Deserialize", "Serialize", "SparqlToProbeConfig", "HashMap", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SparqlQuery!();
        ProbeTemplate!();
    };
}

macro_rules! SparqlToProbeConfig {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct SparqlToProbeConfig { pub queries : Vec < SparqlQuery > , pub probe_templates : HashMap < String , ProbeTemplate > , }
    };
}

SparqlToProbeConfig!();
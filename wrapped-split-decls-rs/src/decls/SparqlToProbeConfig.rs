// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SparqlToProbeConfig",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: ["SparqlQuery", "ProbeTemplate"],
uses: ["Serialize", "SparqlToProbeConfig", "SparqlQuery", "Vec", "Deserialize", "HashMap", "Debug", "ProbeTemplate", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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
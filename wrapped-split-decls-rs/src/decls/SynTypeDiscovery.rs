// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynTypeDiscovery",
decl_type: "function",
source_file: "./src/syn_type_discovery.rs",
source_crate: ".",
deps: [],
uses: ["HashSet", "String", "SynTypeDiscovery", "Default", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SynTypeDiscovery {
    () => {
        # [derive (Debug , Default)] pub struct SynTypeDiscovery { pub discovered_types : HashSet < String > , pub visit_methods : HashSet < String > , pub enum_variants : HashSet < String > , }
    };
}

SynTypeDiscovery!();
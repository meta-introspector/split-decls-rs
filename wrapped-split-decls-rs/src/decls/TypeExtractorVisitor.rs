// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TypeExtractorVisitor",
decl_type: "function",
source_file: "./src/syn_type_discovery.rs",
source_crate: ".",
deps: ["SynTypeDiscovery"],
uses: ["SynTypeDiscovery", "TypeExtractorVisitor"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SynTypeDiscovery!();
    };
}

macro_rules! TypeExtractorVisitor {
    () => {
        deps!();
        struct TypeExtractorVisitor < 'a > { discovery : & 'a mut SynTypeDiscovery , }
    };
}

TypeExtractorVisitor!();
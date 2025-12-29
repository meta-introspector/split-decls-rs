// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TermCollector",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/term_collector.rs",
source_crate: ".",
deps: ["Term"],
uses: ["Debug", "Default", "TermCollector", "Vec", "Term"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Term!();
    };
}

macro_rules! TermCollector {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct TermCollector { pub terms : Vec < Term > , }
    };
}

TermCollector!();
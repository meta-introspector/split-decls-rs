// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TermCollector",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/term_collector.rs",
source_crate: ".",
deps: ["Term"],
uses: ["Default", "TermCollector", "Debug", "Term", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
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
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynMold",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["UsagePattern", "SynSignature", "ComplexityMetrics"],
uses: ["String", "Vec", "UsagePattern", "SynSignature", "HashMap", "SynMold", "ComplexityMetrics", "Clone", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        UsagePattern!();
        SynSignature!();
        ComplexityMetrics!();
    };
}

macro_rules! SynMold {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct SynMold { pub signatures : Vec < SynSignature > , pub complexity_metrics : ComplexityMetrics , pub usage_patterns : HashMap < String , UsagePattern > , }
    };
}

SynMold!();
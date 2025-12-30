// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynMold",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["ComplexityMetrics", "UsagePattern", "SynSignature"],
uses: ["Vec", "SynMold", "Debug", "ComplexityMetrics", "UsagePattern", "Clone", "HashMap", "SynSignature", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ComplexityMetrics!();
        UsagePattern!();
        SynSignature!();
    };
}

macro_rules! SynMold {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct SynMold { pub signatures : Vec < SynSignature > , pub complexity_metrics : ComplexityMetrics , pub usage_patterns : HashMap < String , UsagePattern > , }
    };
}

SynMold!();
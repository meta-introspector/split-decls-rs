// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "UsagePattern",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["PatternType"],
uses: ["Clone", "String", "PatternType", "Debug", "Vec", "UsagePattern", "Usage"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        PatternType!();
    };
}

macro_rules! UsagePattern {
    () => {
        deps!();
        # [doc = " Usage pattern detection"] # [derive (Debug , Clone)] pub struct UsagePattern { pub pattern_type : PatternType , pub frequency : u32 , pub locations : Vec < String > , }
    };
}

UsagePattern!();
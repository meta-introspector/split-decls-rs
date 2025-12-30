// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynUsageVisitor",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["SynOperation"],
uses: ["String", "SynOperation", "HashMap", "Vec", "SynUsageVisitor", "Visitor"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SynOperation!();
    };
}

macro_rules! SynUsageVisitor {
    () => {
        deps!();
        # [doc = " Visitor that extracts syn usage patterns"] pub struct SynUsageVisitor { pub patterns : HashMap < String , Vec < String > > , pub operations : Vec < SynOperation > , pub parse_calls : u32 , pub visit_calls : u32 , pub transform_calls : u32 , pub generation_calls : u32 , }
    };
}

SynUsageVisitor!();
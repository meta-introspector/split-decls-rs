// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynUsageVisitor",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["SynOperation"],
uses: ["Vec", "SynOperation", "Visitor", "HashMap", "String", "SynUsageVisitor"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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
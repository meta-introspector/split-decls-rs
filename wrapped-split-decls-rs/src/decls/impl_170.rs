// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_170",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["SynUsageVisitor"],
uses: ["HashMap", "Vec", "SynUsageVisitor"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SynUsageVisitor!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl SynUsageVisitor { pub fn new () -> Self { Self { patterns : HashMap :: new () , operations : Vec :: new () , parse_calls : 0 , visit_calls : 0 , transform_calls : 0 , generation_calls : 0 , } } }
    };
}

impl_170!();
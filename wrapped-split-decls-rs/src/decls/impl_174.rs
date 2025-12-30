// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_174",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["MoldExtraction"],
uses: ["MoldExtraction", "Generate", "Complexity", "MOLD_ANALYSIS", "Operations", "TokenStream"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        MoldExtraction!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl MoldExtraction { # [doc = " Generate replacement code that uses mold instead of syn"] pub fn generate_replacement (& self) -> TokenStream { let wrapper = & self . mold_wrapper ; let original = & self . original_code ; quote ! { # wrapper # original const _MOLD_ANALYSIS : & str = concat ! ("Complexity: " , stringify ! (# (self . static_analysis . complexity_metrics . total_complexity)) , ", Operations: " , stringify ! (# (self . static_analysis . signatures . len ()))) ; } } }
    };
}

impl_174!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MoldExtraction",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: ["SynMold"],
uses: ["SynMold", "Result", "File", "MoldExtraction", "TokenStream"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SynMold!();
    };
}

macro_rules! MoldExtraction {
    () => {
        deps!();
        # [doc = " Result of mold extraction"] pub struct MoldExtraction { pub original_code : syn :: File , pub mold_wrapper : TokenStream , pub static_analysis : SynMold , }
    };
}

MoldExtraction!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "specialprint",
decl_type: "function",
source_file: "./src/special_print.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! specialprint {
    () => {
        pub fn specialprint (macro_name : & str , local_score : f64) { print ! (", {}: {:.4}" , macro_name , local_score) ; }
    };
}

specialprint!();
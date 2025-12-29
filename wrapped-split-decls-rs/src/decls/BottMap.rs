// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BottMap",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: [],
uses: ["Bott", "K-theory", "BottMap", "The"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! BottMap {
    () => {
        # [doc = " The Bott periodicity map (period 2 for complex K-theory)"] pub struct BottMap ;
    };
}

BottMap!();
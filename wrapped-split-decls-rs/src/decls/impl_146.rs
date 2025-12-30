// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_146",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["Level8DPoint"],
uses: ["None", "Level8DPoint"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Level8DPoint!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl Level8DPoint { pub fn new (coords : [f64 ; 8]) -> Self { Self { coordinates : coords , level : 0 , generation : 0 , cached_result : None , } } }
    };
}

impl_146!();
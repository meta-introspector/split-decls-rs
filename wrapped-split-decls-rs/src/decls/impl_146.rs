// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_146",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["Level8DPoint"],
uses: ["Level8DPoint", "None"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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
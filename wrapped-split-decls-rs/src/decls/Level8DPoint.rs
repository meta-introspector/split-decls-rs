// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Level8DPoint",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Deserialize", "Clone", "String", "Bott", "Option", "Level8DPoint", "Serialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! Level8DPoint {
    () => {
        # [doc = " 8-dimensional point in the Bott tower"] # [derive (Debug , Clone , serde :: Serialize , serde :: Deserialize)] pub struct Level8DPoint { pub coordinates : [f64 ; 8] , pub level : usize , pub generation : usize , pub cached_result : Option < String > , }
    };
}

Level8DPoint!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Level8DPoint",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "Bott", "Deserialize", "String", "Debug", "Serialize", "Level8DPoint", "Option"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! Level8DPoint {
    () => {
        # [doc = " 8-dimensional point in the Bott tower"] # [derive (Debug , Clone , serde :: Serialize , serde :: Deserialize)] pub struct Level8DPoint { pub coordinates : [f64 ; 8] , pub level : usize , pub generation : usize , pub cached_result : Option < String > , }
    };
}

Level8DPoint!();
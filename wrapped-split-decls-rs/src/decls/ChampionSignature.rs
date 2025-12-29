// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ChampionSignature",
decl_type: "function",
source_file: "./src/monster_compressor.rs",
source_crate: ".",
deps: [],
uses: ["Monster", "Vec", "Group", "Debug", "ChampionSignature", "The", "Clone", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! ChampionSignature {
    () => {
        # [doc = " The champion signature with 71 facets (factor of 71 in Monster Group)"] # [derive (Debug , Clone)] pub struct ChampionSignature { pub signature : String , pub facets : Vec < String > , pub monster_order_factor : String , }
    };
}

ChampionSignature!();
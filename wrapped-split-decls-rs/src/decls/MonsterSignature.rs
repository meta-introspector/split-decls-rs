// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MonsterSignature",
decl_type: "function",
source_file: "./src/monster_compressor.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "Vec", "Debug", "MonsterSignature", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! MonsterSignature {
    () => {
        # [derive (Debug , Clone)] pub struct MonsterSignature { pub original : String , pub pairs : Vec < String > , pub triples : Vec < String > , pub pentas : Vec < String > , pub heptas : Vec < String > , pub compressed_form : String , }
    };
}

MonsterSignature!();
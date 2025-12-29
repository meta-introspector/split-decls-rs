// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_187",
decl_type: "function",
source_file: "./src/monster_compressor.rs",
source_crate: ".",
deps: ["MonsterSignature"],
uses: ["Generate", "Monster", "MonsterSignature", "Group", "Calculate", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        MonsterSignature!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl MonsterSignature { # [doc = " Calculate Monster Group compression ratio"] pub fn compression_ratio (& self) -> f64 { let original_size = self . original . len () as f64 ; let compressed_size = self . compressed_form . len () as f64 ; compressed_size / original_size } # [doc = " Generate Monster Group factorization"] pub fn monster_factorization (& self) -> String { format ! ("2^{} × 3^{} × 5^{} × 7^{}" , self . pairs . len () . min (46) , self . triples . len () . min (20) , self . pentas . len () . min (9) , self . heptas . len () . min (6)) } }
    };
}

impl_187!();
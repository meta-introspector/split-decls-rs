// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MonsterCompressor",
decl_type: "function",
source_file: "./src/monster_compressor.rs",
source_crate: ".",
deps: [],
uses: ["One", "String", "Option", "Monster", "Group", "MonsterCompressor", "HashMap", "Frequency", "SINGLETONS"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! MonsterCompressor {
    () => {
        # [doc = " Monster Group order-based signature compression"] # [doc = " 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71"] pub struct MonsterCompressor { # [doc = " 46 most common pairs → emoji (2^46)"] pub pair_emojis : HashMap < String , String > , # [doc = " 20 samples of 3-grams → emoji (3^20)  "] pub triple_emojis : HashMap < String , String > , # [doc = " 9 samples of 5-grams → emoji (5^9)"] pub penta_emojis : HashMap < String , String > , # [doc = " 6 samples of 7-grams → emoji (7^6)"] pub hepta_emojis : HashMap < String , String > , # [doc = " 2 samples of 11-grams → emoji (11^2)"] pub eleven_emojis : HashMap < String , String > , # [doc = " 3 samples of 13-grams → emoji (13^3)"] pub thirteen_emojis : HashMap < String , String > , # [doc = " SINGLETONS: One each for 17, 19, 23, 29, 31, 41, 47, 59, 71"] pub singleton_17 : Option < String > , pub singleton_19 : Option < String > , pub singleton_23 : Option < String > , pub singleton_29 : Option < String > , pub singleton_31 : Option < String > , pub singleton_41 : Option < String > , pub singleton_47 : Option < String > , pub singleton_59 : Option < String > , pub singleton_71 : Option < String > , # [doc = " Frequency tracking"] pub pair_frequencies : HashMap < String , u64 > , pub triple_frequencies : HashMap < String , u64 > , pub penta_frequencies : HashMap < String , u64 > , pub hepta_frequencies : HashMap < String , u64 > , pub eleven_frequencies : HashMap < String , u64 > , pub thirteen_frequencies : HashMap < String , u64 > , }
    };
}

MonsterCompressor!();
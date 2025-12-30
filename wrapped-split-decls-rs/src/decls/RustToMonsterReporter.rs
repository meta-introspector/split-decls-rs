// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RustToMonsterReporter",
decl_type: "function",
source_file: "./src/rust_to_monster_reporter.rs",
source_crate: ".",
deps: ["MonsterCompressor"],
uses: ["Monster", "Generate", "Rust", "Group", "RustToMonsterReporter", "MonsterCompressor"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        MonsterCompressor!();
    };
}

macro_rules! RustToMonsterReporter {
    () => {
        deps!();
        # [doc = " Generate the epic Rust to Monster Group transformation report"] pub struct RustToMonsterReporter { pub compressor : MonsterCompressor , pub total_signatures : usize , pub total_declarations : u64 , pub monster_coverage : f64 , }
    };
}

RustToMonsterReporter!();
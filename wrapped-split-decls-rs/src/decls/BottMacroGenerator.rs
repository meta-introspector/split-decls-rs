// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BottMacroGenerator",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["SuspensionTower"],
uses: ["SuspensionTower", "BottMacroGenerator"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SuspensionTower!();
    };
}

macro_rules! BottMacroGenerator {
    () => {
        deps!();
        pub struct BottMacroGenerator { tower : SuspensionTower , }
    };
}

BottMacroGenerator!();
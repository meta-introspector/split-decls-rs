// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SuspensionTower",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["AbstractionBundle"],
uses: ["Suspending", "AbstractionBundle", "Vec", "SuspensionTower"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        AbstractionBundle!();
    };
}

macro_rules! SuspensionTower {
    () => {
        deps!();
        # [doc = " Suspending 8 times returns you to where you started (up to isomorphism)"] pub struct SuspensionTower { levels : Vec < AbstractionBundle > , }
    };
}

SuspensionTower!();
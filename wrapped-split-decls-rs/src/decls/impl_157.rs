// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_157",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: ["SuspensionTower"],
uses: ["SuspensionTower", "Clone"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SuspensionTower!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl Clone for SuspensionTower { fn clone (& self) -> Self { Self { levels : self . levels . clone () , } } }
    };
}

impl_157!();
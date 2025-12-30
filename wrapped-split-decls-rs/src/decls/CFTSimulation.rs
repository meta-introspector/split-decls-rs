// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CFTSimulation",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: ["ConformalFieldTheory", "ConformalMap", "Level8DPoint"],
uses: ["CFTSimulation", "ConformalFieldTheory", "Debug", "Deserialize", "ConformalMap", "Serialize", "Level8DPoint", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ConformalFieldTheory!();
        ConformalMap!();
        Level8DPoint!();
    };
}

macro_rules! CFTSimulation {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CFTSimulation { pub rustc_cft : ConformalFieldTheory , pub output2_cft : ConformalFieldTheory , pub neutral_space : Level8DPoint , pub conformal_map : ConformalMap , }
    };
}

CFTSimulation!();
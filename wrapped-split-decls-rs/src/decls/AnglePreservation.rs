// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AnglePreservation",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: [],
uses: ["Deserialize", "AnglePreservation", "Debug", "Clone", "Serialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! AnglePreservation {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AnglePreservation { pub c1_angle : f64 , pub c2_angle : f64 , pub preserved : bool , pub error : f64 , }
    };
}

AnglePreservation!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PrimaryField",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: ["SourceArrow"],
uses: ["Serialize", "Deserialize", "Debug", "PrimaryField", "String", "Clone", "SourceArrow"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SourceArrow!();
    };
}

macro_rules! PrimaryField {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PrimaryField { pub name : String , pub conformal_weight : (f64 , f64) , pub source_location : SourceArrow , pub target_location : SourceArrow , }
    };
}

PrimaryField!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ConformalFieldTheory",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: ["CorrelationFunction", "PrimaryField"],
uses: ["Debug", "HashMap", "Serialize", "Vec", "ConformalFieldTheory", "String", "CorrelationFunction", "Clone", "Deserialize", "PrimaryField"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        CorrelationFunction!();
        PrimaryField!();
    };
}

macro_rules! ConformalFieldTheory {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ConformalFieldTheory { pub name : String , pub central_charge : f64 , pub primary_fields : Vec < PrimaryField > , pub correlation_functions : HashMap < String , CorrelationFunction > , }
    };
}

ConformalFieldTheory!();
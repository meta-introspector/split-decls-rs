// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ConformalFieldTheory",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: ["PrimaryField", "CorrelationFunction"],
uses: ["Clone", "Serialize", "ConformalFieldTheory", "Debug", "PrimaryField", "HashMap", "Deserialize", "String", "Vec", "CorrelationFunction"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        PrimaryField!();
        CorrelationFunction!();
    };
}

macro_rules! ConformalFieldTheory {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ConformalFieldTheory { pub name : String , pub central_charge : f64 , pub primary_fields : Vec < PrimaryField > , pub correlation_functions : HashMap < String , CorrelationFunction > , }
    };
}

ConformalFieldTheory!();
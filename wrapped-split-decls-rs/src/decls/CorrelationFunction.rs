// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CorrelationFunction",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Clone", "Serialize", "Deserialize", "CorrelationFunction", "Vec", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! CorrelationFunction {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CorrelationFunction { pub fields : Vec < String > , pub value : f64 , pub conformal_invariant : bool , }
    };
}

CorrelationFunction!();
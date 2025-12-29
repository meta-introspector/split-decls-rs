// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CorrelationFunction",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: [],
uses: ["Deserialize", "Clone", "CorrelationFunction", "String", "Serialize", "Vec", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! CorrelationFunction {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CorrelationFunction { pub fields : Vec < String > , pub value : f64 , pub conformal_invariant : bool , }
    };
}

CorrelationFunction!();
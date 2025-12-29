// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynSignature",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "String", "Debug", "Vec", "Extracted", "SynSignature"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SynSignature {
    () => {
        # [doc = " Extracted signature of syn usage"] # [derive (Debug , Clone)] pub struct SynSignature { pub operation : String , pub input_types : Vec < String > , pub output_types : Vec < String > , pub complexity_score : f64 , pub dependencies : Vec < String > , }
    };
}

SynSignature!();
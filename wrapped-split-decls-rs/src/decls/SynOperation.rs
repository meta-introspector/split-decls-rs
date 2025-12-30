// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynOperation",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: [],
uses: ["String", "SynOperation", "Clone", "Vec", "Debug"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SynOperation {
    () => {
        # [derive (Debug , Clone)] pub struct SynOperation { pub name : String , pub input_types : Vec < String > , pub output_types : Vec < String > , pub complexity : f64 , pub dependencies : Vec < String > , }
    };
}

SynOperation!();
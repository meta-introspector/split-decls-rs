// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MacroBindingSignature",
decl_type: "function",
source_file: "./src/signature_compressor.rs",
source_crate: ".",
deps: [],
uses: ["Macro", "Clone", "MacroBindingSignature", "Vec", "String", "Debug"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! MacroBindingSignature {
    () => {
        # [doc = " Macro binding signature - the string of bindings needed for a declaration"] # [derive (Debug , Clone)] pub struct MacroBindingSignature { pub bindings : Vec < String > , pub signature_string : String , pub prime_key : u64 , pub emoji_key : String , pub frequency : u64 , }
    };
}

MacroBindingSignature!();
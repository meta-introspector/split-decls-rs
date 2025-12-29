// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SignatureLookup",
decl_type: "function",
source_file: "./src/signature_compressor.rs",
source_crate: ".",
deps: [],
uses: ["SignatureLookup", "Signature", "Vec", "HashMap", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SignatureLookup {
    () => {
        # [doc = " Signature lookup table for fast access"] pub struct SignatureLookup { pub prime_to_signature : HashMap < u64 , String > , pub emoji_to_signature : HashMap < String , String > , pub signature_to_bindings : HashMap < String , Vec < String > > , }
    };
}

SignatureLookup!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_181",
decl_type: "function",
source_file: "./src/signature_compressor.rs",
source_crate: ".",
deps: ["SignatureLookup", "SignatureCompressor"],
uses: ["Decode", "Option", "String", "Vec", "HashMap", "SignatureLookup", "SignatureCompressor"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SignatureLookup!();
        SignatureCompressor!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl SignatureLookup { pub fn from_compressor (compressor : & SignatureCompressor) -> Self { let mut prime_to_signature = HashMap :: new () ; let mut emoji_to_signature = HashMap :: new () ; let mut signature_to_bindings = HashMap :: new () ; for (signature , & prime) in & compressor . prime_assignments { prime_to_signature . insert (prime , signature . clone ()) ; } for (signature , emoji) in & compressor . emoji_assignments { emoji_to_signature . insert (emoji . clone () , signature . clone ()) ; } for signature in compressor . frequency_map . keys () { let bindings : Vec < String > = signature . split ('|') . map (| s | s . to_string ()) . collect () ; signature_to_bindings . insert (signature . clone () , bindings) ; } Self { prime_to_signature , emoji_to_signature , signature_to_bindings , } } # [doc = " Decode signature from prime"] pub fn decode_from_prime (& self , prime : u64) -> Option < Vec < String > > { self . prime_to_signature . get (& prime) . and_then (| sig | self . signature_to_bindings . get (sig)) . cloned () } # [doc = " Decode signature from emoji"] pub fn decode_from_emoji (& self , emoji : & str) -> Option < Vec < String > > { self . emoji_to_signature . get (emoji) . and_then (| sig | self . signature_to_bindings . get (sig)) . cloned () } }
    };
}

impl_181!();
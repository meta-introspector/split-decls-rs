// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SignatureCompressor",
decl_type: "function",
source_file: "./src/signature_compressor.rs",
source_crate: ".",
deps: [],
uses: ["String", "Emoji", "Next", "Signature", "Prime", "SignatureCompressor", "HashMap", "Frequency"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SignatureCompressor {
    () => {
        # [doc = " Signature compression system using prime numbers and emojis"] pub struct SignatureCompressor { # [doc = " Prime number assignments (2 = most common, higher primes = rarer)"] pub prime_assignments : HashMap < String , u64 > , # [doc = " Emoji assignments for visual representation"] pub emoji_assignments : HashMap < String , String > , # [doc = " Frequency tracking for optimal prime assignment"] pub frequency_map : HashMap < String , u64 > , # [doc = " Next available prime"] pub next_prime : u64 , }
    };
}

SignatureCompressor!();
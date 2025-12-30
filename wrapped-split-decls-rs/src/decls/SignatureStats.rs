// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SignatureStats",
decl_type: "function",
source_file: "./src/signature_compressor.rs",
source_crate: ".",
deps: [],
uses: ["Statistics", "Debug", "SignatureStats", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SignatureStats {
    () => {
        # [doc = " Statistics about signature compression"] # [derive (Debug)] pub struct SignatureStats { pub total_signatures : usize , pub total_frequency : u64 , pub most_common_signature : String , pub most_common_frequency : u64 , pub least_common_signature : String , pub least_common_frequency : u64 , pub compression_ratio : f64 , pub prime_range : (u64 , u64) , }
    };
}

SignatureStats!();
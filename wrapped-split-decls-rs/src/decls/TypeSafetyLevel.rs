// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TypeSafetyLevel",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: [],
uses: ["Paranoid", "Clone", "Strict", "Serialize", "Permissive", "Deserialize", "Debug", "TypeSafetyLevel"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! TypeSafetyLevel {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum TypeSafetyLevel { Permissive , Strict , Paranoid , }
    };
}

TypeSafetyLevel!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TypeSafetyLevel",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "TypeSafetyLevel", "Debug", "Strict", "Serialize", "Deserialize", "Permissive", "Paranoid"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! TypeSafetyLevel {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum TypeSafetyLevel { Permissive , Strict , Paranoid , }
    };
}

TypeSafetyLevel!();
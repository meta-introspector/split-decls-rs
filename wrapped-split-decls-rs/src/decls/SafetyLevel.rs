// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SafetyLevel",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["Critical", "Debug", "Deserialize", "Clone", "Safe", "Unsafe", "SafetyLevel", "Serialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SafetyLevel {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum SafetyLevel { Safe , Unsafe , Critical , }
    };
}

SafetyLevel!();
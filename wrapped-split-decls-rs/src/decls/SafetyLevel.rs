// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SafetyLevel",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Unsafe", "Clone", "SafetyLevel", "Deserialize", "Safe", "Debug", "Critical"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SafetyLevel {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum SafetyLevel { Safe , Unsafe , Critical , }
    };
}

SafetyLevel!();
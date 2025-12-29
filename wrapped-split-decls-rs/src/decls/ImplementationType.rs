// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ImplementationType",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["Deserialize", "Debug", "Clone", "Logged", "Governed", "Mock", "Production", "Serialize", "ImplementationType"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! ImplementationType {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum ImplementationType { Production , Mock , Logged , Governed , }
    };
}

ImplementationType!();
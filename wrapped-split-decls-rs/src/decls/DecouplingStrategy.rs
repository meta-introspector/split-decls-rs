// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "DecouplingStrategy",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["TraitObject", "GenericBound", "DecouplingStrategy", "Clone", "Debug", "Serialize", "DependencyInject", "Deserialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! DecouplingStrategy {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum DecouplingStrategy { TraitObject , GenericBound , DependencyInject , }
    };
}

DecouplingStrategy!();
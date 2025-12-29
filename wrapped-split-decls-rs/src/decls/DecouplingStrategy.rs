// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "DecouplingStrategy",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Deserialize", "TraitObject", "DependencyInject", "DecouplingStrategy", "Serialize", "Clone", "GenericBound"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! DecouplingStrategy {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum DecouplingStrategy { TraitObject , GenericBound , DependencyInject , }
    };
}

DecouplingStrategy!();
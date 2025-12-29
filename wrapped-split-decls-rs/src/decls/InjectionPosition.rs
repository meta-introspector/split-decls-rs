// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "InjectionPosition",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: [],
uses: ["InjectionPosition", "Serialize", "After", "Replace", "Deserialize", "Inside", "Clone", "Debug", "Before"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! InjectionPosition {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum InjectionPosition { Before , After , Inside , Replace , }
    };
}

InjectionPosition!();
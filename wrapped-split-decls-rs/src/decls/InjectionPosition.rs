// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "InjectionPosition",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: [],
uses: ["Replace", "Deserialize", "Debug", "Clone", "Before", "InjectionPosition", "Serialize", "After", "Inside"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! InjectionPosition {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum InjectionPosition { Before , After , Inside , Replace , }
    };
}

InjectionPosition!();
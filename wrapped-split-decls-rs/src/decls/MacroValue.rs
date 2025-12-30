// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MacroValue",
decl_type: "function",
source_file: "./src/output2_macro_system.rs",
source_crate: ".",
deps: [],
uses: ["MacroValue", "Function", "Enum", "Clone", "Macro", "String", "Struct", "Debug", "Generated"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! MacroValue {
    () => {
        # [derive (Debug , Clone)] pub enum MacroValue { Function (String) , Struct (String) , Enum (String) , Macro (String) , Generated (String) , }
    };
}

MacroValue!();
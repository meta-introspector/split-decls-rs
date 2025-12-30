// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MacroDeclaration",
decl_type: "function",
source_file: "./src/output2_macro_system.rs",
source_crate: ".",
deps: [],
uses: ["MacroDeclaration", "String", "Option", "Debug", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! MacroDeclaration {
    () => {
        # [derive (Debug , Clone)] pub struct MacroDeclaration { pub name : String , pub source_path : String , pub declaration_type : String , pub content : String , pub wrapper : Option < String > , }
    };
}

MacroDeclaration!();
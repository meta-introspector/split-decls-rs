// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MacroDeclaration",
decl_type: "function",
source_file: "./src/output2_macro_system.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Clone", "MacroDeclaration", "String", "Option"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! MacroDeclaration {
    () => {
        # [derive (Debug , Clone)] pub struct MacroDeclaration { pub name : String , pub source_path : String , pub declaration_type : String , pub content : String , pub wrapper : Option < String > , }
    };
}

MacroDeclaration!();
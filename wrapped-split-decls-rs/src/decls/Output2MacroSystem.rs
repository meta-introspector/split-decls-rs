// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Output2MacroSystem",
decl_type: "function",
source_file: "./src/output2_macro_system.rs",
source_crate: ".",
deps: ["LispInterpreter", "MacroDeclaration"],
uses: ["Import", "String", "Creates", "Lisp-like", "HashMap", "Runtime", "Output2MacroSystem", "LispInterpreter", "All", "MacroDeclaration", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        LispInterpreter!();
        MacroDeclaration!();
    };
}

macro_rules! Output2MacroSystem {
    () => {
        deps!();
        # [doc = " Import all output2 declarations as callable macros"] # [doc = " Creates a Lisp-like runtime system for code generation"] # [derive (Debug)] pub struct Output2MacroSystem { # [doc = " All available macros from output2 declarations"] pub macros : HashMap < String , MacroDeclaration > , # [doc = " Runtime interpreter for macro calls"] pub interpreter : LispInterpreter , }
    };
}

Output2MacroSystem!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "LispInterpreter",
decl_type: "function",
source_file: "./src/output2_macro_system.rs",
source_crate: ".",
deps: ["MacroValue"],
uses: ["LispInterpreter", "String", "HashMap", "Debug", "MacroValue"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        MacroValue!();
    };
}

macro_rules! LispInterpreter {
    () => {
        deps!();
        # [derive (Debug)] pub struct LispInterpreter { pub environment : HashMap < String , MacroValue > , }
    };
}

LispInterpreter!();
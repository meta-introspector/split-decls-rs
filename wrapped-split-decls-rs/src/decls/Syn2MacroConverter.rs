// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Syn2MacroConverter",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["BottMacroGenerator", "SecurityContext"],
uses: ["Option", "Syn2MacroConverter", "BottMacroGenerator", "Bott", "SecurityContext", "Main"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        BottMacroGenerator!();
        SecurityContext!();
    };
}

macro_rules! Syn2MacroConverter {
    () => {
        deps!();
        # [doc = " Main syn2macro converter with Bott periodicity awareness"] pub struct Syn2MacroConverter { security : SecurityContext , bott_generator : Option < BottMacroGenerator > , }
    };
}

Syn2MacroConverter!();
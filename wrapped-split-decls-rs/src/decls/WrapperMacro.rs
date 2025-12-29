// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "WrapperMacro",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["DecouplingStrategy"],
uses: ["Debug", "Serialize", "WrapperMacro", "String", "Deserialize", "DecouplingStrategy"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        DecouplingStrategy!();
    };
}

macro_rules! WrapperMacro {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct WrapperMacro { pub name : String , pub trait_name : String , pub decoupling_strategy : DecouplingStrategy , }
    };
}

WrapperMacro!();
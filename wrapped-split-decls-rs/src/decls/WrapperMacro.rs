// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "WrapperMacro",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["DecouplingStrategy"],
uses: ["Serialize", "WrapperMacro", "Deserialize", "DecouplingStrategy", "String", "Debug"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
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